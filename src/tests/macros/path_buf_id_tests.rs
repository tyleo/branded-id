use crate::{PathBufId, path_buf_id, tests::util::BTest};
use std::path::PathBuf;

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
