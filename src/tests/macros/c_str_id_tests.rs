use crate::{CStrId, c_str_id, tests::util::BTest};

#[test]
fn c_str_id_0_test() {
    let actual: &CStrId<BTest> = c_str_id!(c"a");
    let expected = CStrId::from_c_str(c"a");
    assert_eq!(actual, expected);
}

#[test]
fn c_str_id_1_test() {
    let actual: &CStrId<BTest> = c_str_id!(BTest; c"a");
    let expected = CStrId::from_c_str(c"a");
    assert_eq!(actual, expected);
}
