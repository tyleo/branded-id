use crate::{IdVec, id_vec, tests::util::BTest};

#[test]
fn id_vec_0_test() {
    let actual = id_vec![BTest; i32];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_1_test() {
    let actual = id_vec![BTest; i32; 2];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_2_test() {
    let actual = id_vec![BTest; i32; 2; 2];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_3_test() {
    let actual: IdVec<_, i32> = id_vec![BTest];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_4_test() {
    let actual = id_vec![BTest; 2];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_5_test() {
    let actual = id_vec![BTest; 2; 2];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![2; 2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_6_test() {
    let actual: IdVec<_, i32> = id_vec![];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_7_test() {
    let actual = id_vec![2];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![2]);
    assert_eq!(actual, expected);
}

#[test]
fn id_vec_8_test() {
    let actual = id_vec![2; 2];
    let expected: IdVec<BTest, i32> = IdVec::from_vec(vec![2; 2]);
    assert_eq!(actual, expected);
}
