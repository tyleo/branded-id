use crate::{StrId, str_id, tests::util::BTest};

#[test]
fn str_id_0_test() {
    let actual: &StrId<BTest> = str_id!("a");
    let expected = StrId::from_str("a");
    assert_eq!(actual, expected);
}

#[test]
fn str_id_1_test() {
    let actual: &StrId<BTest> = str_id!(BTest; "a");
    let expected = StrId::from_str("a");
    assert_eq!(actual, expected);
}
