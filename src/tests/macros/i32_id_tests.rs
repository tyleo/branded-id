use crate::{I32Id, i32_id, tests::util::MTest};

#[test]
fn i32_id_0_test() {
    let actual: I32Id<MTest> = i32_id!(1);
    let expected = I32Id::from_i32(1);
    assert_eq!(actual, expected);
}

#[test]
fn i32_id_1_test() {
    let actual: I32Id<MTest> = i32_id!(MTest; 1);
    let expected = I32Id::from_i32(1);
    assert_eq!(actual, expected);
}
