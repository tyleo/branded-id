use crate::{IdArray, id_array, tests::util::MTest};

#[test]
fn id_array_0_test() {
    let actual = id_array![MTest; i32];
    let expected: IdArray<MTest, i32, 0> = IdArray::from_array([]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_1_test() {
    let actual = id_array![MTest; i32; 2];
    let expected: IdArray<MTest, i32, 1> = IdArray::from_array([2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_2_test() {
    let actual = id_array![MTest; i32; 2; 2];
    let expected: IdArray<MTest, i32, 2> = IdArray::from_array([2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_3_test() {
    let actual: IdArray<_, i32, 0> = id_array![MTest];
    let expected: IdArray<MTest, i32, 0> = IdArray::from_array([]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_4_test() {
    let actual = id_array![MTest; 2];
    let expected: IdArray<MTest, i32, 1> = IdArray::from_array([2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_5_test() {
    let actual = id_array![MTest; 2; 2];
    let expected: IdArray<MTest, i32, 2> = IdArray::from_array([2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_6_test() {
    let actual: IdArray<MTest, i32, 0> = id_array![];
    let expected: IdArray<MTest, i32, 0> = IdArray::from_array([]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_7_test() {
    let actual = id_array![2];
    let expected: IdArray<MTest, i32, 1> = IdArray::from_array([2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_array_8_test() {
    let actual = id_array![2; 2];
    let expected: IdArray<MTest, i32, 2> = IdArray::from_array([2; 2]);
    assert_eq!(actual, expected);
}
