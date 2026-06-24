use crate::{U8Id, ext::U8Ext, tests::util::BTest, u8_id as id};

#[test]
fn to_u8_id_test() {
    let u8 = 1u8;

    let actual: U8Id<BTest> = u8.to_u8_id();
    let expected = id!(u8);
    assert_eq!(actual, expected);
}
