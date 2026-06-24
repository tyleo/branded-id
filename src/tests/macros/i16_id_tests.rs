use crate::{I16Id, i16_id, tests::util::BTest};

#[test]
fn i16_id_0_test() {
    let actual: I16Id<BTest> = i16_id!(1);
    let expected = I16Id::from_i16(1);
    assert_eq!(actual, expected);
}

#[test]
fn i16_id_1_test() {
    let actual: I16Id<BTest> = i16_id!(BTest; 1);
    let expected = I16Id::from_i16(1);
    assert_eq!(actual, expected);
}
