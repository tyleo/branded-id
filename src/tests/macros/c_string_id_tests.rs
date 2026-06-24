use crate::{CStringId, c_string_id, tests::util::BTest};
use std::ffi::CString;

#[test]
fn c_string_id_0_test() {
    let actual: CStringId<BTest> = c_string_id!(c"a");
    let expected = CStringId::from_c_string(CString::from(c"a"));
    assert_eq!(actual, expected);
}

#[test]
fn c_string_id_1_test() {
    let actual: CStringId<BTest> = c_string_id!(BTest; c"a");
    let expected = CStringId::from_c_string(CString::from(c"a"));
    assert_eq!(actual, expected);
}
