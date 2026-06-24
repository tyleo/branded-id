use crate::{I128Id, i128_id, tests::util::BTest};

#[test]
fn i128_id_0_test() {
    let actual: I128Id<BTest> = i128_id!(1);
    let expected = I128Id::from_i128(1);
    assert_eq!(actual, expected);
}

#[test]
fn i128_id_1_test() {
    let actual: I128Id<BTest> = i128_id!(BTest; 1);
    let expected = I128Id::from_i128(1);
    assert_eq!(actual, expected);
}
