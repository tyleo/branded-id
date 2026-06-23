use crate::{
    IdArray, id_array, id_slice,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_as_test() {
    let id_array = id_array![BTestBase; 0, 1, 2];

    let actual: &IdArray<BTest, i32, 3> = id_array.downcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_test() {
    let id_array = id_array![BTest; 0, 1, 2];

    let actual: &IdArray<BTestBase, i32, 3> = id_array.upcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_as_mut_test() {
    let mut id_array = id_array![BTestBase; 0, 1, 2];

    let actual: &IdArray<BTest, i32, 3> = id_array.downcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_mut_test() {
    let mut id_array = id_array![BTest; 0, 1, 2];

    let actual: &IdArray<BTestBase, i32, 3> = id_array.upcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_into_test() {
    let id_array = id_array![BTestBase; 0, 1, 2];

    let actual: IdArray<BTest, i32, 3> = id_array.downcast_into();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_into_test() {
    let id_array = id_array![BTest; 0, 1, 2];

    let actual: IdArray<BTestBase, i32, 3> = id_array.upcast_into();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}
