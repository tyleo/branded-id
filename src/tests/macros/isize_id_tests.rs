use crate::{IsizeId, isize_id, tests::util::BTest};

#[test]
fn isize_id_0_test() {
    let actual: IsizeId<BTest> = isize_id!(1);
    let expected = IsizeId::from_isize(1);
    assert_eq!(actual, expected);
}

#[test]
fn isize_id_1_test() {
    let actual: IsizeId<BTest> = isize_id!(BTest; 1);
    let expected = IsizeId::from_isize(1);
    assert_eq!(actual, expected);
}
