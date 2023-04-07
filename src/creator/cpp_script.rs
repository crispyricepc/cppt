use std::path::PathBuf;

use super::{
    traits::{dirname::Dirname, namespaced_creator::NamespacedCreator, src_creator::SrcCreator},
    Creator,
};
use crate::format::clang_format;
use anyhow::Result;

pub struct CppScriptCreator {
    name: String,
    namespace: Option<String>,
}

impl Creator for CppScriptCreator {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: None,
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn contents(&self) -> Result<Vec<(PathBuf, String)>> {
        Ok(vec![(self.src_filepath(), self.src_content()?)])
    }
}

impl SrcCreator for CppScriptCreator {
    fn src_content(&self) -> Result<String> {
        clang_format(format!(
            "#include <iostream>

            {}
            int main(int argc, char** argv) {{
                std::cout << \"Hello, world!\" << std::endl;
                return 0;
            }}
            {}
            ",
            match &self.namespace {
                Some(ns) => format!("namespace {} {{", ns),
                None => "".to_string(),
            },
            match &self.namespace {
                Some(ns) => format!("}} // namespace {}", ns),
                None => "".to_string(),
            }
        ))
    }

    fn src_dirname(&self) -> PathBuf {
        self.directory()
    }
}

impl NamespacedCreator for CppScriptCreator {
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
    use crate::tests::create_and_assert;

    #[test]
    fn filenames_ok() {
        let creator = CppScriptCreator::new("script").with_namespace("test_namespace");
        assert_eq!(
            creator.src_filepath(),
            PathBuf::from("./src/test_namespace/script.cpp")
        );
        assert_eq!(creator.src_dirname(), PathBuf::from("./src/test_namespace"));

        let results = creator.contents().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, creator.src_filepath());
    }

    #[test]
    fn creator_script() {
        create_and_assert(
            CppScriptCreator::new("script").with_namespace("test_namespace"),
            &[PathBuf::from("./src/test_namespace/script.cpp")],
        );
    }

    #[test]
    fn creator_script_no_namespace() {
        create_and_assert(
            CppScriptCreator::new("script"),
            &[PathBuf::from("./src/script.cpp")],
        );
    }
}
