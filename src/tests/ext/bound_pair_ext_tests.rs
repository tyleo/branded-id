use crate::{IdSlice, id_slice, tests::util::BTest, usize_id as id};
use std::ops::{Bound, Index, IndexMut};

#[test]
fn get_test() {
    let bound_pair = (
        Bound::Included(id!(BTest; 0)),
        Bound::Included(id!(BTest; 1)),
    );
    let id_slice = id_slice![BTest; 1, 2, 3, 4];

    let actual = id_slice.get(bound_pair);
    let expected = Some(id_slice![BTest; 1, 2]);
    assert_eq!(actual, expected);
}

#[test]
fn get_mut_test() {
    let bound_pair = (
        Bound::Included(id!(BTest; 0)),
        Bound::Included(id!(BTest; 1)),
    );
    let mut slice = [1, 2, 3, 4];
    let id_slice = IdSlice::from_mut_slice(&mut slice);

    let actual = id_slice.get_mut(bound_pair);
    let mut expected = [1, 2];
    let expected = Some(IdSlice::from_mut_slice(&mut expected));
    assert_eq!(actual, expected);
}

#[test]
fn index_test() {
    let bound_pair = (
        Bound::Included(id!(BTest; 0)),
        Bound::Included(id!(BTest; 1)),
    );
    let id_slice = id_slice![BTest; 1, 2, 3, 4];

    let actual = id_slice.index(bound_pair);
    let expected = id_slice![BTest; 1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn index_mut_test() {
    let bound_pair = (
        Bound::Included(id!(BTest; 0)),
        Bound::Included(id!(BTest; 1)),
    );
    let mut slice = [1, 2, 3, 4];
    let id_slice = IdSlice::from_mut_slice(&mut slice);

    let actual = id_slice.index_mut(bound_pair);
    let mut expected = [1, 2];
    let expected = IdSlice::from_mut_slice(&mut expected);
    assert_eq!(actual, expected);
}
