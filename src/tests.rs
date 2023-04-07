use crate::creator::Creator;
use pretty_assertions::{assert_eq, assert_str_eq};
use std::{fs::File, io::Read, path::PathBuf};

#[inline]
pub fn assert_str_file_eq(actual: String, expected_path: &str) {
    let mut f = File::open(expected_path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    assert_str_eq!(actual, s);
}

pub fn create_and_assert(creator: impl Creator, expected_files: &[PathBuf]) {
    let res = creator.contents();
    assert!(res.is_ok());
    let res = res.unwrap();

    assert_eq!(res.len(), expected_files.len());
    for (filepath, content) in &res {
        assert!(expected_files.contains(filepath));
        assert_str_eq!(
            content,
            &File::open(PathBuf::from("examples").join(filepath))
                .unwrap()
                .bytes()
                .map(|b| b.unwrap() as char)
                .collect::<String>()
        );
    }
}
