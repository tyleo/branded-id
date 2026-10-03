use crate::{PathBufId, path_buf_id, tests::util::BTest};
use std::{ffi::OsString, path::PathBuf};

#[test]
fn path_buf_id_0_test() {
    let actual: PathBufId<BTest> = path_buf_id!("a");
    let expected = PathBufId::from_path_buf(PathBuf::from("a"));
    assert_eq!(actual, expected);
}

#[test]
fn path_buf_id_1_test() {
    let actual: PathBufId<BTest> = path_buf_id!(BTest; "a");
    let expected = PathBufId::from_path_buf(PathBuf::from("a"));
    assert_eq!(actual, expected);
}

#[test]
fn path_buf_id_of_an_os_string_test() {
    let actual: PathBufId<BTest> = path_buf_id!(BTest; OsString::from("a"));
    let expected = PathBufId::from_path_buf(PathBuf::from("a"));
    assert_eq!(actual, expected);
}
