use crate::{ext::U32Ext, tests::util::MTest, u32_id as id, U32Id};

#[test]
fn to_u32_id_test() {
    let u32 = 1u32;

    let actual: U32Id<MTest> = u32.to_u32_id();
    let expected = id!(u32);
    assert_eq!(actual, expected);
}
