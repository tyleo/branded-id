use crate::{I32Id, i32_id, tests::util::BTest};

#[test]
fn i32_id_0_test() {
    let actual: I32Id<BTest> = i32_id!(1);
    let expected = I32Id::from_i32(1);
    assert_eq!(actual, expected);
}

#[test]
fn i32_id_1_test() {
    let actual: I32Id<BTest> = i32_id!(BTest; 1);
    let expected = I32Id::from_i32(1);
    assert_eq!(actual, expected);
}
