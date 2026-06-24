use crate::{U64Id, tests::util::BTest, u64_id};

#[test]
fn u64_id_0_test() {
    let actual: U64Id<BTest> = u64_id!(1);
    let expected = U64Id::from_u64(1);
    assert_eq!(actual, expected);
}

#[test]
fn u64_id_1_test() {
    let actual: U64Id<BTest> = u64_id!(BTest; 1);
    let expected = U64Id::from_u64(1);
    assert_eq!(actual, expected);
}
