use std::path::PathBuf;

use super::{
    traits::{
        dirname::Dirname, header_creator::HeaderCreator, namespaced_creator::NamespacedCreator,
        src_creator::SrcCreator,
    },
    Creator,
};
use crate::format::clang_format;
use anyhow::Result;
use convert_case::{Case, Casing};

pub struct CppClassCreator {
    name: String,
    namespace: Option<String>,
}

impl Creator for CppClassCreator {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_case(Case::Pascal),
            namespace: None,
        }
    }

    fn contents(&self) -> Result<Vec<(PathBuf, String)>> {
        Ok(vec![
            (self.src_filepath(), self.src_content()?),
            (self.header_filepath(), self.header_content()?),
        ])
    }

    fn name(&self) -> &String {
        &self.name
    }
}

impl SrcCreator for CppClassCreator {
    fn src_dirname(&self) -> PathBuf {
        self.directory()
    }

    fn src_content(&self) -> Result<String> {
        clang_format(format!(
            "#include \"{0}\"

            {2}
            {1}::{1}() {{
            }}
            {1}::~{1}() {{
            }}
            {3}
            ",
            self.header_basename(),
            self.name,
            match self.namespace() {
                Some(ns) => format!("namespace {} {{", ns),
                None => "".to_string(),
            },
            match self.namespace() {
                Some(ns) => format!("}} // namespace {}", ns),
                None => "".to_string(),
            }
        ))
    }
}

impl HeaderCreator for CppClassCreator {
    fn header_dirname(&self) -> PathBuf {
        self.directory()
    }

    fn header_content(&self) -> Result<String> {
        clang_format(format!(
            "#pragma once

            {0}
            class {1} {{
            public:
                {1}();
                ~{1}();
            }};
            {2}
            ",
            if let Some(namespace) = &self.namespace {
                format!("namespace {} {{", namespace)
            } else {
                "".to_string()
            },
            self.name,
            if let Some(namespace) = &self.namespace {
                format!("}} // namespace {}", namespace)
            } else {
                "".to_string()
            }
        ))
    }
}

impl NamespacedCreator for CppClassCreator {
    fn with_namespace(self, namespace: &str) -> Self {
        Self {
            namespace: Some(namespace.to_string()),
            ..self
        }
    }

    fn namespace(&self) -> &Option<String> {
        &self.namespace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_str_file_eq;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn filenames_ok() {
        let creator = CppClassCreator::new("Class");
        assert_eq!(creator.src_filepath(), PathBuf::from("./src/class.cpp"));
        assert_str_eq!(creator.src_basename(), "class.cpp");
        assert_eq!(creator.header_filepath(), PathBuf::from("./src/class.hpp"));
        assert_str_eq!(creator.header_basename(), "class.hpp");
    }

    #[test]
    fn namespaced_filenames_ok() {
        let creator = CppClassCreator::new("class").with_namespace("test_namespace");
        assert_eq!(
            creator.src_filepath(),
            PathBuf::from("./src/test_namespace/class.cpp")
        );
        assert_str_eq!(creator.src_basename(), "class.cpp");
        assert_eq!(
            creator.header_filepath(),
            PathBuf::from("./src/test_namespace/class.hpp")
        );
        assert_str_eq!(creator.header_basename(), "class.hpp");
    }

    #[test]
    fn src_contents_ok() {
        let creator = CppClassCreator::new("class");
        assert_str_file_eq(creator.src_content().unwrap(), "examples/src/class.cpp");
    }

    #[test]
    fn namespaced_src_contents_ok() {
        let creator = CppClassCreator::new("Class").with_namespace("test_namespace");
        assert_str_file_eq(
            creator.src_content().unwrap(),
            "examples/src/test_namespace/class.cpp",
        );
    }

    #[test]
    fn header_contents_okay() {
        let creator = CppClassCreator::new("Class");
        assert_str_file_eq(creator.header_content().unwrap(), "examples/src/class.hpp");
    }

    #[test]
    fn namespaced_header_contents_okay() {
        let creator = CppClassCreator::new("Class").with_namespace("test_namespace");
        assert_str_file_eq(
            creator.header_content().unwrap(),
            "examples/src/test_namespace/class.hpp",
        );
    }
}
