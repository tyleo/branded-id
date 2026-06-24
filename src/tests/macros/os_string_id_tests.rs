use crate::{OsStringId, os_string_id, tests::util::BTest};
use std::ffi::OsString;

#[test]
fn os_string_id_0_test() {
    let actual: OsStringId<BTest> = os_string_id!("a");
    let expected = OsStringId::from_os_string(OsString::from("a"));
    assert_eq!(actual, expected);
}

#[test]
fn os_string_id_1_test() {
    let actual: OsStringId<BTest> = os_string_id!(BTest; "a");
    let expected = OsStringId::from_os_string(OsString::from("a"));
    assert_eq!(actual, expected);
}
