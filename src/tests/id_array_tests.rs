use crate::{id_array, id_slice, tests::MTest, usize_id as id, IdArray};

#[test]
fn from_array_test() {
    let array = [1];

    let actual: IdArray<MTest, i32, 1> = IdArray::from(array);
    let expected = id_array![MTest; i32; 1];
    assert_eq!(actual, expected);
}

#[test]
fn from_array_ref_test() {
    let array = &[1];

    let actual: &IdArray<MTest, i32, 1> = IdArray::from_array_ref(array);
    let expected = &id_array![MTest; i32; 1];
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_array_test() {
    let array = &mut [1];

    let actual: &mut IdArray<MTest, i32, 1> = IdArray::from_mut_array(array);
    actual[id!(0)] = 2;

    let expected = &mut id_array![MTest; i32; 2];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_id_slice() {
    let id_array = &mut id_array![1];

    let actual = id_array.as_mut_id_slice();
    actual[id!(0)] = 2;

    let expected = id_slice![MTest; i32; 2];
    assert_eq!(actual, expected);
}

#[test]
fn as_id_slice() {
    let id_array = &id_array![1];

    let actual = id_array.as_id_slice();
    let expected = id_slice![MTest; i32; 1];
    assert_eq!(actual, expected);
}

#[test]
fn as_ref_test() {
    let id_array = id_array![1];

    let actual = id_array.as_ref();
    let expected = id_slice![MTest; 1];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_test() {
    let mut id_array = id_array![1];

    let actual = id_array.as_mut();
    actual[id!(0)] = 2;

    let expected = id_slice![MTest; 2];
    assert_eq!(actual, expected);
}

#[test]
fn from_test() {
    let array = [1];

    let actual: IdArray<MTest, i32, 1> = From::from(array);
    let expected = id_array![MTest; i32; 1];
    assert_eq!(actual, expected);
}

#[test]
fn index_test() {
    let id_array = id_array![MTest; 1];

    let id = id!(0);

    let actual = id_array[id];
    let expected = 1;

    assert_eq!(actual, expected);
}

#[test]
fn index_mut_test() {
    let mut id_array = id_array![MTest; 1];

    let id = id!(0);
    id_array[id] = 2;

    let actual = id_array[id];
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_array_0 = id_array![MTest; 1];
    let id_array_1 = id_array![1];

    let actual = id_array_0 == id_array_1;
    let expected = true;
    assert_eq!(actual, expected);
}
