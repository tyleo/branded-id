use crate::{I64Id, ext::I64Ext, i64_id as id, tests::util::BTest};

#[test]
fn to_i64_id_test() {
    let i64 = 1i64;

    let actual: I64Id<BTest> = i64.to_i64_id();
    let expected = id!(i64);
    assert_eq!(actual, expected);
}
