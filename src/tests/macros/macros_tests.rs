use crate::{
    id::{tests::marker::MUnknown, Id32, Id32I, IdArray, IdSlice, IdVec},
    id32, id32i, id_array, id_slice, id_vec, mut_id_slice,
};

#[test]
fn id_array_0_test() {
    let actual = id_array![MUnknown; i32];
    let expected: IdArray<MUnknown, i32, 0> = IdArray::from_array([]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_1_test() {
    let actual = id_array![MUnknown; i32; 2];
    let expected: IdArray<MUnknown, i32, 1> = IdArray::from_array([2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_2_test() {
    let actual = id_array![MUnknown; i32; 2; 2];
    let expected: IdArray<MUnknown, i32, 2> = IdArray::from_array([2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_3_test() {
    let actual = id_array![MUnknown];
    let expected: IdArray<MUnknown, i32, 0> = IdArray::from_array([]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_4_test() {
    let actual = id_array![MUnknown; 2];
    let expected: IdArray<MUnknown, i32, 1> = IdArray::from_array([2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_5_test() {
    let actual = id_array![MUnknown; 2; 2];
    let expected: IdArray<MUnknown, i32, 2> = IdArray::from_array([2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_6_test() {
    let actual = id_array![];
    let expected: IdArray<MUnknown, i32, 0> = IdArray::from_array([]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_7_test() {
    let actual = id_array![2];
    let expected: IdArray<MUnknown, i32, 1> = IdArray::from_array([2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_8_test() {
    let actual = id_array![2; 2];
    let expected: IdArray<MUnknown, i32, 2> = IdArray::from_array([2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_0_test() {
    let actual = id_slice![MUnknown; i32];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_1_test() {
    let actual = id_slice![MUnknown; i32; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_2_test() {
    let actual = id_slice![MUnknown; i32; 2; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_3_test() {
    let actual = id_slice![MUnknown];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_4_test() {
    let actual = id_slice![MUnknown; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_5_test() {
    let actual = id_slice![MUnknown; 2; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_6_test() {
    let actual = id_slice![];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_7_test() {
    let actual = id_slice![2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_8_test() {
    let actual = id_slice![2; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_0_test() {
    let actual = id_vec![MUnknown; i32];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_1_test() {
    let actual = id_vec![MUnknown; i32; 2];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_2_test() {
    let actual = id_vec![MUnknown; i32; 2; 2];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_3_test() {
    let actual = id_vec![MUnknown];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_4_test() {
    let actual = id_vec![MUnknown; 2];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_5_test() {
    let actual = id_vec![MUnknown; 2; 2];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_6_test() {
    let actual = id_vec![];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_7_test() {
    let actual = id_vec![2];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_8_test() {
    let actual = id_vec![2; 2];
    let expected: IdVec<MUnknown, i32> = IdVec::from_vec(vec![2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_0_test() {
    let actual = mut_id_slice![MUnknown; i32];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_1_test() {
    let actual = mut_id_slice![MUnknown; i32; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_2_test() {
    let actual = mut_id_slice![MUnknown; i32; 2; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_3_test() {
    let actual = mut_id_slice![MUnknown];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_4_test() {
    let actual = mut_id_slice![MUnknown; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_5_test() {
    let actual = mut_id_slice![MUnknown; 2; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_6_test() {
    let actual = mut_id_slice![];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_7_test() {
    let actual = mut_id_slice![2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_slice_8_test() {
    let actual = mut_id_slice![2; 2];
    let expected: &IdSlice<MUnknown, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}
