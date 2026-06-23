use crate::{IdSlice, id_slice, tests::util::BTest};

#[test]
fn id_slice_0_test() {
    let actual = id_slice![BTest; i32];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_1_test() {
    let actual = id_slice![BTest; i32; 2];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_2_test() {
    let actual = id_slice![BTest; i32; 2; 2];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_3_test() {
    let actual: &IdSlice<_, i32> = id_slice![BTest];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_4_test() {
    let actual = id_slice![BTest; 2];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_5_test() {
    let actual = id_slice![BTest; 2; 2];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_6_test() {
    let actual: &IdSlice<_, i32> = id_slice![];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_7_test() {
    let actual = id_slice![2];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_8_test() {
    let actual = id_slice![2; 2];
    let expected: &IdSlice<BTest, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}
