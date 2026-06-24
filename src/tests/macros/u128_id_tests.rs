use crate::{U128Id, tests::util::BTest, u128_id};

#[test]
fn u128_id_0_test() {
    let actual: U128Id<BTest> = u128_id!(1);
    let expected = U128Id::from_u128(1);
    assert_eq!(actual, expected);
}

#[test]
fn u128_id_1_test() {
    let actual: U128Id<BTest> = u128_id!(BTest; 1);
    let expected = U128Id::from_u128(1);
    assert_eq!(actual, expected);
}
