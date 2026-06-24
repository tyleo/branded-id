use crate::{I8Id, i8_id, tests::util::BTest};

#[test]
fn i8_id_0_test() {
    let actual: I8Id<BTest> = i8_id!(1);
    let expected = I8Id::from_i8(1);
    assert_eq!(actual, expected);
}

#[test]
fn i8_id_1_test() {
    let actual: I8Id<BTest> = i8_id!(BTest; 1);
    let expected = I8Id::from_i8(1);
    assert_eq!(actual, expected);
}
