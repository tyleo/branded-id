use crate::{UsizeId, tests::util::MTest, usize_id};

#[test]
fn usize_id_0_test() {
    let actual: UsizeId<MTest> = usize_id!(1);
    let expected = UsizeId::from_usize(1);
    assert_eq!(actual, expected);
}

#[test]
fn usize_id_1_test() {
    let actual: UsizeId<MTest> = usize_id!(MTest; 1);
    let expected = UsizeId::from_usize(1);
    assert_eq!(actual, expected);
}
