use crate::{UsizeId, tests::util::BTest, usize_id};

#[test]
fn usize_id_0_test() {
    let actual: UsizeId<BTest> = usize_id!(1);
    let expected = UsizeId::from_usize(1);
    assert_eq!(actual, expected);
}

#[test]
fn usize_id_1_test() {
    let actual: UsizeId<BTest> = usize_id!(BTest; 1);
    let expected = UsizeId::from_usize(1);
    assert_eq!(actual, expected);
}
