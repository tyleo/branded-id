use crate::{
    IdSlice, id_slice,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_as_test() {
    let id_slice = id_slice![BTestBase; 0, 1, 2];

    let actual: &IdSlice<BTest, i32> = id_slice.downcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_test() {
    let id_slice = id_slice![BTest; 0, 1, 2];

    let actual: &IdSlice<BTestBase, i32> = id_slice.upcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_as_mut_test() {
    let mut array = [0, 1, 2];
    let id_slice = IdSlice::<BTestBase, _>::from_mut_slice(&mut array);

    let actual: &IdSlice<BTest, i32> = id_slice.downcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_mut_test() {
    let mut array = [0, 1, 2];
    let id_slice = IdSlice::<BTest, _>::from_mut_slice(&mut array);

    let actual: &IdSlice<BTestBase, i32> = id_slice.upcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}
