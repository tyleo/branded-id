use crate::{
    IdVec, id_slice, id_vec,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_as_test() {
    let id_vec = id_vec![BTestBase; 0, 1, 2];

    let actual: &IdVec<BTest, i32> = id_vec.downcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_test() {
    let id_vec = id_vec![BTest; 0, 1, 2];

    let actual: &IdVec<BTestBase, i32> = id_vec.upcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_as_mut_test() {
    let mut id_vec = id_vec![BTestBase; 0, 1, 2];

    let actual: &IdVec<BTest, i32> = id_vec.downcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_mut_test() {
    let mut id_vec = id_vec![BTest; 0, 1, 2];

    let actual: &IdVec<BTestBase, i32> = id_vec.upcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_into_test() {
    let id_vec = id_vec![BTestBase; 0, 1, 2];

    let actual: IdVec<BTest, i32> = id_vec.downcast_into();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_into_test() {
    let id_vec = id_vec![BTest; 0, 1, 2];

    let actual: IdVec<BTestBase, i32> = id_vec.upcast_into();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}
