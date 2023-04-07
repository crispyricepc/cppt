use std::path::PathBuf;

use crate::creator::Creator;
use anyhow::Result;
use convert_case::{Case, Casing};

pub trait SrcCreator: Creator {
    fn src_dirname(&self) -> PathBuf {
        self.directory()
    }
    fn src_content(&self) -> Result<String>;
    fn src_basename(&self) -> String {
        format!(
            "{}.cpp",
            self.name()
                .to_case(Case::Snake)
                .to_case(Case::Lower)
                .replace(" ", "_")
        )
    }
    fn src_filepath(&self) -> PathBuf {
        self.src_dirname().join(self.src_basename())
    }
}
