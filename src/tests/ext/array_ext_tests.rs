use crate::{IdArray, ext::ArrayExt, id_array, tests::util::BTest};

#[test]
fn as_id_array_test() {
    let array = [1, 2, 3];

    let actual: &IdArray<BTest, i32, 3> = array.as_id_array();
    let expected = &id_array![1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_id_array_test() {
    let mut array = [1, 2, 3];

    let actual: &mut IdArray<BTest, i32, 3> = array.as_mut_id_array();
    let expected = &id_array![1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn into_id_array_test() {
    let array = [1, 2, 3];

    let actual: IdArray<BTest, i32, 3> = array.into_id_array();
    let expected = id_array![1, 2, 3];
    assert_eq!(actual, expected);
}
