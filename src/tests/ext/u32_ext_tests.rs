use crate::{U32Id, ext::U32Ext, tests::util::BTest, u32_id as id};

#[test]
fn to_u32_id_test() {
    let u32 = 1u32;

    let actual: U32Id<BTest> = u32.to_u32_id();
    let expected = id!(u32);
    assert_eq!(actual, expected);
}
