use self::traits::dirname::Dirname;
use anyhow::Result;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

pub mod cpp_class;
pub mod cpp_script;
pub mod traits;

pub trait Creator: Dirname {
    fn new(name: &str) -> Self;
    fn name(&self) -> &String;
    fn save(&self) -> Result<()> {
        for (filepath, content) in self.contents()? {
            if filepath.exists() {
                return Err(anyhow::anyhow!("File already exists"));
            }
            create_dir_all(filepath.parent().unwrap())?;
            let mut f = File::create(filepath)?;
            f.write_all(content.as_bytes())?;
        }

        Ok(())
    }
    fn contents(&self) -> Result<Vec<(PathBuf, String)>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;
    use std::{fs::remove_file, io::Read, path::Path};

    struct DummyCreator {
        name: String,
    }
    impl Creator for DummyCreator {
        fn name(&self) -> &String {
            &self.name
        }

        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
            }
        }

        fn contents(&self) -> Result<Vec<(PathBuf, String)>> {
            Ok(vec![(PathBuf::from("dummy.txt"), "Hello".to_string())])
        }
    }
    impl Dirname for DummyCreator {}

    #[test]
    fn create_file() {
        let creator = DummyCreator::new("dummy");
        assert!(creator.save().is_ok());

        let p = Path::new("dummy.txt");
        assert!(p.exists());
        assert!(p.is_file());

        let mut f = File::open(p).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        assert_str_eq!(s, "Hello");

        remove_file(p).unwrap();
    }

    #[test]
    fn file_already_exists() {
        let creator = DummyCreator::new("dummy");
        assert!(creator.save().is_ok());
        assert!(creator.save().is_err());

        let p = Path::new("dummy.txt");
        remove_file(p).unwrap();
    }
}
