use crate::{IdVec, ext::VecExt, id_vec, tests::util::BTest};

#[test]
fn as_id_vec_test() {
    let vec = vec![1, 2, 3];

    let actual: &IdVec<BTest, i32> = vec.as_id_vec();
    let expected = &id_vec![1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_id_vec_test() {
    let mut vec = vec![1, 2, 3];

    let actual: &mut IdVec<BTest, i32> = vec.as_mut_id_vec();
    let expected = &id_vec![1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn into_id_vec_test() {
    let vec = vec![1, 2, 3];

    let actual: IdVec<BTest, i32> = vec.into_id_vec();
    let expected = id_vec![1, 2, 3];
    assert_eq!(actual, expected);
}
