use crate::{U16Id, tests::util::BTest, u16_id};

#[test]
fn u16_id_0_test() {
    let actual: U16Id<BTest> = u16_id!(1);
    let expected = U16Id::from_u16(1);
    assert_eq!(actual, expected);
}

#[test]
fn u16_id_1_test() {
    let actual: U16Id<BTest> = u16_id!(BTest; 1);
    let expected = U16Id::from_u16(1);
    assert_eq!(actual, expected);
}
