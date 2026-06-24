use crate::{U8Id, tests::util::BTest, u8_id};

#[test]
fn u8_id_0_test() {
    let actual: U8Id<BTest> = u8_id!(1);
    let expected = U8Id::from_u8(1);
    assert_eq!(actual, expected);
}

#[test]
fn u8_id_1_test() {
    let actual: U8Id<BTest> = u8_id!(BTest; 1);
    let expected = U8Id::from_u8(1);
    assert_eq!(actual, expected);
}
