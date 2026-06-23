use crate::{IsizeId, isize_id, tests::util::MTest};

#[test]
fn isize_id_0_test() {
    let actual: IsizeId<MTest> = isize_id!(1);
    let expected = IsizeId::from_isize(1);
    assert_eq!(actual, expected);
}

#[test]
fn isize_id_1_test() {
    let actual: IsizeId<MTest> = isize_id!(MTest; 1);
    let expected = IsizeId::from_isize(1);
    assert_eq!(actual, expected);
}
