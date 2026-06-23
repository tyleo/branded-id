use crate::{IdSlice, ext::SliceExt, id_slice, tests::util::MTest};

#[test]
fn as_id_slice_test() {
    let slice = &[1, 2, 3];

    let actual: &IdSlice<MTest, i32> = slice.as_id_slice();
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_id_slice_test() {
    let slice = &mut [1, 2, 3];

    let actual: &mut IdSlice<MTest, i32> = slice.as_mut_id_slice();
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);
}
