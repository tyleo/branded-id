use crate::{
    IdVec, id_slice, id_vec,
    tests::util::{MTest, extends::MTestBase},
};

#[test]
fn downcast_as_test() {
    let id_vec = id_vec![MTestBase; 0, 1, 2];

    let actual: &IdVec<MTest, i32> = id_vec.downcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_test() {
    let id_vec = id_vec![MTest; 0, 1, 2];

    let actual: &IdVec<MTestBase, i32> = id_vec.upcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_as_mut_test() {
    let mut id_vec = id_vec![MTestBase; 0, 1, 2];

    let actual: &IdVec<MTest, i32> = id_vec.downcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_mut_test() {
    let mut id_vec = id_vec![MTest; 0, 1, 2];

    let actual: &IdVec<MTestBase, i32> = id_vec.upcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_into_test() {
    let id_vec = id_vec![MTestBase; 0, 1, 2];

    let actual: IdVec<MTest, i32> = id_vec.downcast_into();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_into_test() {
    let id_vec = id_vec![MTest; 0, 1, 2];

    let actual: IdVec<MTestBase, i32> = id_vec.upcast_into();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}
