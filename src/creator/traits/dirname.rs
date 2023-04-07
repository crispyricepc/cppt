use std::path::PathBuf;

pub trait Dirname {
    fn directory(&self) -> PathBuf {
        PathBuf::from(".")
    }
}
