use crate::{StringId, string_id, tests::util::BTest};

#[test]
fn string_id_0_test() {
    let actual: StringId<BTest> = string_id!("a");
    let expected = StringId::from_string(String::from("a"));
    assert_eq!(actual, expected);
}

#[test]
fn string_id_1_test() {
    let actual: StringId<BTest> = string_id!(BTest; "a");
    let expected = StringId::from_string(String::from("a"));
    assert_eq!(actual, expected);
}
