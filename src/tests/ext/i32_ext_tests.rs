use crate::{I32Id, ext::I32Ext, i32_id as id, tests::util::BTest};

#[test]
fn to_i32_id_test() {
    let i32 = 1i32;

    let actual: I32Id<BTest> = i32.to_i32_id();
    let expected = id!(i32);
    assert_eq!(actual, expected);
}
