use crate::{
    IdArray, id_array, id_slice,
    tests::util::{MTest, extends::MTestBase},
};

#[test]
fn downcast_as_test() {
    let id_array = id_array![MTestBase; 0, 1, 2];

    let actual: &IdArray<MTest, i32, 3> = id_array.downcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_test() {
    let id_array = id_array![MTest; 0, 1, 2];

    let actual: &IdArray<MTestBase, i32, 3> = id_array.upcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_as_mut_test() {
    let mut id_array = id_array![MTestBase; 0, 1, 2];

    let actual: &IdArray<MTest, i32, 3> = id_array.downcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_mut_test() {
    let mut id_array = id_array![MTest; 0, 1, 2];

    let actual: &IdArray<MTestBase, i32, 3> = id_array.upcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_into_test() {
    let id_array = id_array![MTestBase; 0, 1, 2];

    let actual: IdArray<MTest, i32, 3> = id_array.downcast_into();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_into_test() {
    let id_array = id_array![MTest; 0, 1, 2];

    let actual: IdArray<MTestBase, i32, 3> = id_array.upcast_into();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}
