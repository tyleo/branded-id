use crate::{IdSlice, SliceExt, id_slice, tests::util::BTest, u32_id as id};

#[test]
fn as_id_slice_test() {
    let slice = &[1, 2, 3];

    let actual: &IdSlice<BTest, i32> = slice.as_id_slice();
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_id_slice_test() {
    let slice = &mut [1, 2, 3];

    let actual: &mut IdSlice<BTest, i32> = slice.as_mut_id_slice();
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn display_ids_test() {
    let slice = &[id!(BTest; 4), id!(BTest; 5)];

    let actual = slice.display_ids().to_string();
    let expected = "[4, 5]";
    assert_eq!(actual, expected);
}
