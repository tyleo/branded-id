use crate::{id_slice, tests::util::MTest, IdSlice};

#[test]
fn id_slice_0_test() {
    let actual = id_slice![MTest; i32];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_1_test() {
    let actual = id_slice![MTest; i32; 2];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_2_test() {
    let actual = id_slice![MTest; i32; 2; 2];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_3_test() {
    let actual: &IdSlice<_, i32> = id_slice![MTest];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_4_test() {
    let actual = id_slice![MTest; 2];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_5_test() {
    let actual = id_slice![MTest; 2; 2];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_6_test() {
    let actual: &IdSlice<_, i32> = id_slice![];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_7_test() {
    let actual = id_slice![2];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_slice_8_test() {
    let actual = id_slice![2; 2];
    let expected: &IdSlice<MTest, i32> = IdSlice::from_slice(&[2; 2]);
    assert_eq!(actual, expected);
}
