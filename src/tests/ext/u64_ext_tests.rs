use crate::{U64Id, ext::U64Ext, tests::util::BTest, u64_id as id};

#[test]
fn to_u64_id_test() {
    let u64 = 1u64;

    let actual: U64Id<BTest> = u64.to_u64_id();
    let expected = id!(u64);
    assert_eq!(actual, expected);
}
