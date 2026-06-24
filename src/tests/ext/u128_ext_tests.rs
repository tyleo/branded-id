use crate::{U128Id, ext::U128Ext, tests::util::BTest, u128_id as id};

#[test]
fn to_u128_id_test() {
    let u128 = 1u128;

    let actual: U128Id<BTest> = u128.to_u128_id();
    let expected = id!(u128);
    assert_eq!(actual, expected);
}
