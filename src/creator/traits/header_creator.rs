use std::path::PathBuf;

use crate::creator::Creator;
use anyhow::Result;
use convert_case::{Case, Casing};

pub trait HeaderCreator: Creator {
    fn header_dirname(&self) -> PathBuf;
    fn header_content(&self) -> Result<String>;
    fn header_basename(&self) -> String {
        format!(
            "{}.hpp",
            self.name()
                .to_case(Case::Snake)
                .to_case(Case::Lower)
                .replace(" ", "_")
        )
    }
    fn header_filepath(&self) -> PathBuf {
        self.header_dirname().join(self.header_basename())
    }
}
