use crate::{U16Id, ext::U16Ext, tests::util::BTest, u16_id as id};

#[test]
fn to_u16_id_test() {
    let u16 = 1u16;

    let actual: U16Id<BTest> = u16.to_u16_id();
    let expected = id!(u16);
    assert_eq!(actual, expected);
}
