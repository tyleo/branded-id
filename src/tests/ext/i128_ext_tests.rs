use crate::{I128Id, ext::I128Ext, i128_id as id, tests::util::BTest};

#[test]
fn to_i128_id_test() {
    let i128 = 1i128;

    let actual: I128Id<BTest> = i128.to_i128_id();
    let expected = id!(i128);
    assert_eq!(actual, expected);
}
