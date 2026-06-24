use crate::{I64Id, i64_id, tests::util::BTest};

#[test]
fn i64_id_0_test() {
    let actual: I64Id<BTest> = i64_id!(1);
    let expected = I64Id::from_i64(1);
    assert_eq!(actual, expected);
}

#[test]
fn i64_id_1_test() {
    let actual: I64Id<BTest> = i64_id!(BTest; 1);
    let expected = I64Id::from_i64(1);
    assert_eq!(actual, expected);
}
