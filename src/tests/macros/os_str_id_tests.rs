use crate::{OsStrId, os_str_id, tests::util::BTest};
use std::ffi::OsStr;

#[test]
fn os_str_id_0_test() {
    let actual: &OsStrId<BTest> = os_str_id!("a");
    let expected = OsStrId::from_os_str(OsStr::new("a"));
    assert_eq!(actual, expected);
}

#[test]
fn os_str_id_1_test() {
    let actual: &OsStrId<BTest> = os_str_id!(BTest; "a");
    let expected = OsStrId::from_os_str(OsStr::new("a"));
    assert_eq!(actual, expected);
}
