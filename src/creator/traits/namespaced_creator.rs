use std::path::PathBuf;

use crate::creator::Creator;

use super::dirname::Dirname;

pub trait NamespacedCreator: Creator {
    fn with_namespace(self, namespace: &str) -> Self;
    fn namespace(&self) -> &Option<String>;
    fn namespace_path(&self) -> Option<PathBuf> {
        match self.namespace() {
            Some(namespace) => Some(PathBuf::from(namespace.replace("::", "/"))),
            None => None,
        }
    }
}

impl<T: NamespacedCreator> Dirname for T {
    fn directory(&self) -> PathBuf {
        match self.namespace_path() {
            Some(path) => PathBuf::from("./src").join(path),
            None => PathBuf::from("./src"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_str_eq;
    use std::{
        fs::{remove_dir_all, remove_file, File},
        io::Read,
        path::Path,
    };

    struct DummyCreator {
        namespace: Option<String>,
    }

    impl Creator for DummyCreator {
        fn name(&self) -> &String {
            panic!()
        }
        fn new(_: &str) -> Self {
            panic!()
        }
        fn contents(&self) -> Result<Vec<(PathBuf, String)>> {
            Ok(vec![(
                self.directory().join("dummy.txt"),
                "Hello".to_string(),
            )])
        }
    }
    impl NamespacedCreator for DummyCreator {
        fn with_namespace(self, namespace: &str) -> Self {
            DummyCreator {
                namespace: Some(namespace.to_string()),
            }
        }

        fn namespace(&self) -> &Option<String> {
            &self.namespace
        }
    }

    #[test]
    fn namespace_path_single() {
        let creator = DummyCreator {
            namespace: Some("foo".to_string()),
        };
        assert!(creator.namespace_path().is_some());
        assert_eq!(creator.namespace_path().unwrap(), PathBuf::from("foo"));
    }

    #[test]
    fn namespace_path_double() {
        let creator = DummyCreator {
            namespace: Some("foo::bar".to_string()),
        };
        assert!(creator.namespace_path().is_some());
        assert_eq!(creator.namespace_path().unwrap(), PathBuf::from("foo/bar"));
    }

    #[test]
    fn namespace_path_triple() {
        let creator = DummyCreator {
            namespace: Some("foo::bar::baz".to_string()),
        };
        assert!(creator.namespace_path().is_some());
        assert_eq!(
            creator.namespace_path().unwrap(),
            PathBuf::from("foo/bar/baz")
        );
    }

    #[test]
    fn namespace_path_none() {
        let creator = DummyCreator { namespace: None };
        assert!(creator.namespace_path().is_none());
    }

    #[test]
    fn create_namespaced_file() {
        let creator = DummyCreator {
            namespace: Some("foo::bar::baz".to_string()),
        };
        assert!(creator.save().is_ok());

        let dir = PathBuf::from("src/foo/bar/baz");
        let p = dir.join("dummy.txt");
        assert!(p.exists());
        assert!(p.is_file());

        let mut f = File::open(&p).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        assert_str_eq!(s, "Hello");

        remove_file(p).unwrap();
        remove_dir_all("src/foo").unwrap();
    }
}
