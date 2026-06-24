use crate::{PathId, path_id, tests::util::BTest};
use std::path::Path;

#[test]
fn path_id_0_test() {
    let actual: &PathId<BTest> = path_id!("a");
    let expected = PathId::from_path(Path::new("a"));
    assert_eq!(actual, expected);
}

#[test]
fn path_id_1_test() {
    let actual: &PathId<BTest> = path_id!(BTest; "a");
    let expected = PathId::from_path(Path::new("a"));
    assert_eq!(actual, expected);
}
