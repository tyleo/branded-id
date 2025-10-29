use crate::{ext::I32Ext, i32_id as id, tests::util::MTest, I32Id};

#[test]
fn to_i32_id_test() {
    let i32 = 1i32;

    let actual: I32Id<MTest> = i32.to_i32_id();
    let expected = id!(i32);
    assert_eq!(actual, expected);
}
