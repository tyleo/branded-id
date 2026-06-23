use crate::{
    id_slice,
    tests::util::{extends::MTestBase, MTest},
    IdSlice,
};

#[test]
fn downcast_as_test() {
    let id_slice = id_slice![MTestBase; 0, 1, 2];

    let actual: &IdSlice<MTest, i32> = id_slice.downcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_test() {
    let id_slice = id_slice![MTest; 0, 1, 2];

    let actual: &IdSlice<MTestBase, i32> = id_slice.upcast_as();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn downcast_as_mut_test() {
    let mut array = [0, 1, 2];
    let id_slice = IdSlice::<MTestBase, _>::from_mut_slice(&mut array);

    let actual: &IdSlice<MTest, i32> = id_slice.downcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn upcast_as_mut_test() {
    let mut array = [0, 1, 2];
    let id_slice = IdSlice::<MTest, _>::from_mut_slice(&mut array);

    let actual: &IdSlice<MTestBase, i32> = id_slice.upcast_as_mut();
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);
}
