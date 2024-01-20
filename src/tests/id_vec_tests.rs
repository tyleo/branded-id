use crate::{
    id::{tests::marker::MUnknown, GetIdSlice, GetIdSliceMut, Id32, IdVec},
    id32, id_vec,
};

#[test]
fn new_test() {
    let actual = IdVec::new();
    let expected = id_vec!(MUnknown; i32);
    assert_eq!(actual, expected);
}

#[test]
fn from_vec_test() {
    let vec = vec![1];
    let id_vec = IdVec::from_vec(vec);

    let id = id32!(MUnknown; 0);

    let actual = id_vec[id];
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn is_empty_test() {
    let mut id_vec = id_vec![MUnknown];

    let actual = id_vec.is_empty();
    let expected = true;
    assert_eq!(actual, expected);

    id_vec.push(0);

    let actual = id_vec.is_empty();
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn end_test() {
    let mut id_vec = id_vec![MUnknown];

    let actual = id_vec.end();
    let expected = id32!(0);
    assert_eq!(actual, expected);

    id_vec.push(0);

    let actual = id_vec.end();
    let expected = id32!(1);
    assert_eq!(actual, expected);
}

#[test]
fn len_test() {
    let mut id_vec = id_vec![MUnknown];

    let actual = id_vec.len();
    let expected = 0;
    assert_eq!(actual, expected);

    id_vec.push(0);

    let actual = id_vec.len();
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn push_test() {
    let mut id_vec = id_vec![MUnknown];

    let actual_idx = id_vec.push(1);
    let expected_idx = id32!(0);
    assert_eq!(actual_idx, expected_idx);

    let actual_value = id_vec[actual_idx];
    let expected_value = 1;
    assert_eq!(actual_value, expected_value);
}

#[test]
fn resize_test() {
    let mut id_vec = id_vec![MUnknown];
    id_vec.resize(5, 1);

    let actual = id_vec;
    let expected = id_vec![MUnknown; 1, 1, 1, 1, 1];
    assert_eq!(actual, expected);
}

#[test]
fn get_id_slice_test() {
    let id_vec = id_vec![MUnknown; 1];
    let id_slice = id_vec.get_id_slice();

    let id = id32!(0);

    let actual = id_slice[id];
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn get_id_slice_mut_test() {
    let mut id_vec = id_vec![MUnknown; 1];
    let id_slice = id_vec.get_id_slice_mut();

    let id = id32!(0);
    id_slice[id] = 2;

    let actual = id_slice[id];
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn default_test() {
    let id_vec: IdVec<MUnknown, i32> = Default::default();

    let actual = id_vec.len();
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_0 = id_vec![MUnknown; 0];
    let id_1 = id_vec![MUnknown; 1];
    let id_2 = id_vec![MUnknown; 0];

    let actual = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_0 == id_2;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn index_test() {
    let id_vec = id_vec![MUnknown; 1];

    let id = id32!(0);

    let actual = id_vec[id];
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn index_mut_test() {
    let mut id_vec = id_vec![MUnknown; 1];

    let id = id32!(0);
    id_vec[id] = 2;

    let actual = id_vec[id];
    let expected = 2;
    assert_eq!(actual, expected);
}
