use crate::{
    IdSlice, IdVec, UsizeId, id_array, id_slice, id_vec, tests::util::MTest, usize_id as id,
};
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut, Index, IndexMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

#[test]
fn as_mut_id_slice_test() {
    let mut vec = id_vec![MTest; 1];

    let actual: &mut IdSlice<MTest, i32> = vec.as_mut_id_slice();
    actual[id!(0)] = 2;

    let expected = &mut [2];
    let expected = IdSlice::from_mut_slice(expected);
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_vec_test() {
    let mut vec = id_vec![MTest; 1];

    let actual: &mut Vec<i32> = vec.as_mut_vec();
    actual[0] = 2;

    let expected = &vec![2];
    assert_eq!(actual, expected);
}

#[test]
fn as_id_slice_test() {
    let vec = id_vec![MTest; 1];

    let actual: &IdSlice<MTest, i32> = vec.as_id_slice();
    let expected = id_slice!(1);
    assert_eq!(actual, expected);
}

#[test]
fn as_vec_test() {
    let vec = id_vec![MTest; 1];

    let actual: &Vec<i32> = vec.as_vec();
    let expected = &vec![1];
    assert_eq!(actual, expected);
}

#[test]
fn end_test() {
    let vec = id_vec![MTest; 1];

    let actual: UsizeId<MTest> = vec.end();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_vec_test() {
    let vec = &mut vec![1];

    let actual: &mut IdVec<MTest, i32> = IdVec::from_mut_vec(vec);
    actual[id!(0)] = 2;

    let expected = id_slice![2];
    assert_eq!(actual, expected);
}

#[test]
fn from_vec_test() {
    let vec = vec![1];

    let actual: IdVec<MTest, i32> = IdVec::from_vec(vec);
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn from_vec_ref_test() {
    let vec = vec![1];

    let actual: &IdVec<MTest, i32> = IdVec::from_vec_ref(&vec);
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn into_vec_test() {
    let vec = id_vec![MTest; 1];

    let actual: Vec<i32> = vec.into_vec();
    let expected = &[1];
    assert_eq!(actual, expected);
}

#[test]
fn is_empty_test() {
    let id_vec = id_vec![MTest; i32];

    let actual: bool = id_vec.is_empty();
    let expected = true;
    assert_eq!(actual, expected);

    let id_vec = id_vec![MTest; 1];

    let actual: bool = id_vec.is_empty();
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn len_test() {
    let id_vec = id_vec![MTest; i32];

    let actual: usize = id_vec.len();
    let expected = 0;
    assert_eq!(actual, expected);

    let id_vec = id_vec![MTest; 1];

    let actual: usize = id_vec.len();
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn new_test() {
    let actual: IdVec<MTest, i32> = IdVec::new();
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn push_test() {
    let mut id_vec = id_vec![MTest];

    let actual: UsizeId<MTest> = id_vec.push(1);
    let expected = id!(0);
    assert_eq!(actual, expected);

    let actual: IdVec<MTest, i32> = id_vec;
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn resize_test() {
    let mut id_vec = id_vec![MTest];
    id_vec.resize(5, 1);

    let actual: IdVec<MTest, i32> = id_vec;
    let expected = id_slice![1, 1, 1, 1, 1];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::useless_asref)]
fn id_slice_as_mut_test() {
    let mut id_vec = id_vec![MTest; 1];

    let actual: &mut IdSlice<MTest, i32> = id_vec.as_mut();
    actual[id!(0)] = 2;

    let expected = id_slice![2];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::useless_asref)]
fn id_vec_as_mut_test() {
    let mut id_vec = id_vec![MTest; 1];

    let actual: &mut IdVec<MTest, i32> = id_vec.as_mut();
    actual[id!(0)] = 2;

    let expected = id_slice![2];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::useless_asref)]
fn id_slice_as_ref_test() {
    let id_vec = &id_vec![MTest; 1];

    let actual: &IdSlice<MTest, i32> = id_vec.as_ref();
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::useless_asref)]
fn id_vec_as_ref_test() {
    let id_vec = id_vec![MTest; 1];

    let actual: &IdVec<MTest, i32> = id_vec.as_ref();
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn borrow_test() {
    let id_vec = &id_vec![MTest; 1];

    let actual: &IdSlice<MTest, i32> = id_vec.borrow();
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn borrow_mut_test() {
    let mut id_vec = id_vec![MTest; 1];

    let actual: &mut IdSlice<MTest, i32> = id_vec.borrow_mut();
    actual[id!(0)] = 2;

    let expected = id_slice![2];
    assert_eq!(actual, expected);
}

#[test]
fn clone_test() {
    let id_array = id_vec!(MTest; 1);

    let actual: IdVec<MTest, i32> = id_array.clone();
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn fmt_test() {
    let empty_id_vec = id_vec![MTest; i32];
    let actual: String = format!("{:?}", empty_id_vec);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:10?}", empty_id_vec);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>10?}", empty_id_vec);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^10?}", empty_id_vec);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:.1?}", empty_id_vec);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", empty_id_vec);
    let expected = "id_sys::tests::util::m_test::MTest[]";
    assert_eq!(actual, expected);

    let nested_id_vec = IdVec::<MTest, _>::from_vec(vec![["Hello"], ["Hi"]]);

    let actual: String = format!("{:#?}", nested_id_vec);
    let expected = "id_sys::tests::util::m_test::MTest[\n    [\n        \"Hello\",\n    ],\n    [\n        \"Hi\",\n    ],\n]";
    assert_eq!(actual, expected);

    let id_vec = id_vec![MTest; "Hello", "Hi"];

    let actual: String = format!("{:?}", id_vec);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:10?}", id_vec);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>10?}", id_vec);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^10?}", id_vec);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:.1?}", id_vec);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id_vec);
    let expected = "id_sys::tests::util::m_test::MTest[\n    \"Hello\",\n    \"Hi\",\n]";
    assert_eq!(actual, expected);
}

#[test]
fn default_test() {
    let actual: IdVec<MTest, i32> = Default::default();
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn deref_test() {
    let id_vec = &id_vec![MTest; 1];

    let actual: &IdSlice<MTest, i32> = id_vec.deref();
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn deref_mut_test() {
    let mut id_vec = id_vec![MTest; 1];

    let actual: &mut IdSlice<MTest, i32> = id_vec.deref_mut();
    actual[id!(0)] = 2;

    let expected = id_slice![2];
    assert_eq!(actual, expected);
}

#[test]
fn extend_copy_test() {
    let mut actual = id_vec![MTest; 1];
    actual.extend([&1]);

    let expected = id_slice![1, 1];
    assert_eq!(actual, expected);
}

#[test]
fn extend_test() {
    let mut actual = id_vec![MTest; 1];
    actual.extend([1]);

    let expected = id_slice![1, 1];
    assert_eq!(actual, expected);
}

#[test]
fn from_test() {
    let vec = vec![1];

    let actual: IdVec<MTest, i32> = From::from(vec);
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn from_iter_test() {
    let array = [1];

    let actual: IdVec<MTest, i32> = FromIterator::from_iter(array);
    let expected = id_slice![1];
    assert_eq!(actual, expected);
}

#[test]
fn hash_test() {
    let id = id_vec![MTest; 1];
    let mut hasher_0 = DefaultHasher::new();
    id.hash(&mut hasher_0);

    let int = vec![1];
    let mut hasher_1 = DefaultHasher::new();
    int.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn hash_slice_test() {
    let ids = [id_vec![MTest; 1], id_vec![MTest; 2]];
    let mut hasher_0 = DefaultHasher::new();
    ids.hash(&mut hasher_0);

    let ints = [vec![1], vec![2]];
    let mut hasher_1 = DefaultHasher::new();
    ints.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::redundant_slicing)]
fn index_test() {
    let id_vec = id_vec![MTest; 0, 1, 2, 3];

    let actual: &i32 = id_vec.index(id!(1));
    let expected = &1;
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_vec.index(id!(1)..id!(3));
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_vec.index(id!(1)..);
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_vec.index(..);
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_vec.index(id!(1)..=id!(3));
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_vec.index(..id!(3));
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_vec.index(..=id!(3));
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::redundant_slicing)]
fn index_mut_test() {
    let mut id_vec = id_vec![MTest; 0, 1, 2, 3];

    let actual: &mut i32 = id_vec.index_mut(id!(1));
    let expected = &1;
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_vec.index_mut(id!(1)..id!(3));
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_vec.index_mut(id!(1)..);
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_vec.index_mut(..);
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_vec.index_mut(id!(1)..=id!(3));
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_vec.index_mut(..id!(3));
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_vec.index_mut(..=id!(3));
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn into_iter_test() {
    let id_vec = id_vec![MTest; 0, 1, 2];

    let actual: IntoIter<i32> = id_vec.into_iter();
    let expected = [0, 1, 2].into_iter();
    assert!(actual.eq(expected));
}

#[test]
fn into_iter_ref_test() {
    let id_vec = id_vec![MTest; 0, 1, 2];

    let actual: Iter<i32> = id_vec.iter();
    let expected = [0, 1, 2].iter();
    assert!(actual.eq(expected));
}

#[test]
fn into_iter_mut_test() {
    let mut id_vec = id_vec![MTest; 0, 1, 2];

    let actual: IterMut<i32> = id_vec.iter_mut();
    let expected = [0, 1, 2].iter();
    assert!(actual.eq(expected));
}

#[test]
fn cmp_test() {
    let vec_0 = id_vec![MTest; 1];
    let vec_1 = id_vec![MTest; 2];

    let actual: Ordering = vec_0.cmp(&vec_0);
    let expected = Ordering::Equal;
    assert_eq!(actual, expected);

    let actual: Ordering = vec_0.cmp(&vec_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Ordering = vec_1.cmp(&vec_0);
    let expected = Ordering::Greater;
    assert_eq!(actual, expected);
}

#[test]
fn max_test() {
    let vec_0 = id_vec![MTest; 1];
    let vec_1 = id_vec![MTest; 2];

    let actual: IdVec<MTest, i32> = vec_0.max(vec_1);
    let expected = id_vec![MTest; 2];
    assert_eq!(actual, expected);
}

#[test]
fn min_test() {
    let vec_0 = id_vec![MTest; 1];
    let vec_1 = id_vec![MTest; 2];

    let actual: IdVec<MTest, i32> = vec_0.min(vec_1);
    let expected = id_vec![MTest; 1];
    assert_eq!(actual, expected);
}

#[test]
fn clamp_test() {
    let vec_0 = id_vec![MTest; 1];
    let vec_1 = id_vec![MTest; 2];
    let vec_2 = id_vec![MTest; 3];

    let actual: IdVec<MTest, i32> = vec_2.clamp(vec_0, vec_1);
    let expected = id_vec![MTest; 2];
    assert_eq!(actual, expected);

    let vec_0 = id_vec![MTest; 1];
    let vec_1 = id_vec![MTest; 2];
    let vec_2 = id_vec![MTest; 3];

    let actual: IdVec<MTest, i32> = vec_0.clamp(vec_1, vec_2);
    let expected = id_vec![MTest; 2];
    assert_eq!(actual, expected);
}

#[test]
fn eq_slice_ref_test() {
    let id_vec = id_vec![MTest; 1];
    let id_slice_0 = id_slice![1];
    let id_slice_1 = id_slice![2];

    let actual: bool = id_vec.eq(&id_slice_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.eq(&id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_slice_ref_test() {
    let id_vec = id_vec![MTest; 1];
    let id_slice_0 = id_slice![1];
    let id_slice_1 = id_slice![2];

    let actual: bool = id_vec.ne(&id_slice_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.ne(&id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn eq_array_ref_test() {
    let id_vec = id_vec![MTest; 1];
    let id_array_0 = &id_array![1];
    let id_array_1 = &id_array![2];

    let actual: bool = id_vec.eq(&id_array_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.eq(&id_array_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_array_ref_test() {
    let id_vec = id_vec![MTest; 1];
    let id_array_0 = &id_array![1];
    let id_array_1 = &id_array![2];

    let actual: bool = id_vec.ne(&id_array_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.ne(&id_array_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn eq_slice_mut_test() {
    let id_vec = id_vec![MTest; 1];
    let mut slice_0 = [1];
    let id_slice_0 = IdSlice::from_mut_slice(&mut slice_0);
    let mut slice_1 = [2];
    let id_slice_1 = IdSlice::from_mut_slice(&mut slice_1);

    let actual: bool = id_vec.eq(&id_slice_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.eq(&id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_slice_mut_test() {
    let id_vec = id_vec![MTest; 1];
    let mut slice_0 = [1];
    let id_slice_0 = IdSlice::from_mut_slice(&mut slice_0);
    let mut slice_1 = [2];
    let id_slice_1 = IdSlice::from_mut_slice(&mut slice_1);

    let actual: bool = id_vec.ne(&id_slice_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.ne(&id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn eq_slice_test() {
    let id_vec = id_vec![MTest; 1];
    let id_slice_0 = id_slice![1];
    let id_slice_1 = id_slice![2];

    let actual: bool = id_vec.eq(id_slice_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.eq(id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_slice_test() {
    let id_vec = id_vec![MTest; 1];
    let id_slice_0 = id_slice![1];
    let id_slice_1 = id_slice![2];

    let actual: bool = id_vec.ne(id_slice_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.ne(id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn eq_array_test() {
    let id_vec = id_vec![MTest; 1];
    let id_array_0 = id_array![1];
    let id_array_1 = id_array![2];

    let actual: bool = id_vec.eq(&id_array_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.eq(&id_array_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_array_test() {
    let id_vec = id_vec![MTest; 1];
    let id_array_0 = id_array![1];
    let id_array_1 = id_array![2];

    let actual: bool = id_vec.ne(&id_array_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec.ne(&id_array_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let id_vec_0 = id_vec!(MTest; 1);
    let id_vec_1 = id_vec!(MTest; 2);

    let actual: Option<Ordering> = id_vec_0.partial_cmp(&id_vec_0);
    let expected = Some(Ordering::Equal);
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_vec_0.partial_cmp(&id_vec_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_vec_1.partial_cmp(&id_vec_0);
    let expected = Some(Ordering::Greater);
    assert_eq!(actual, expected);
}

#[test]
fn lt_test() {
    let id_vec_0 = id_vec!(MTest; 1);
    let id_vec_1 = id_vec!(MTest; 2);

    let actual: bool = id_vec_0 < id_vec_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec_0 < id_vec_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec_1 < id_vec_0;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn le_test() {
    let id_vec_0 = id_vec!(MTest; 1);
    let id_vec_1 = id_vec!(MTest; 2);

    let actual: bool = id_vec_0 <= id_vec_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec_0 <= id_vec_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec_1 <= id_vec_0;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn gt_test() {
    let id_vec_0 = id_vec!(MTest; 1);
    let id_vec_1 = id_vec!(MTest; 2);

    let actual: bool = id_vec_0 > id_vec_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec_0 > id_vec_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec_1 > id_vec_0;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn ge_test() {
    let id_vec_0 = id_vec!(MTest; 1);
    let id_vec_1 = id_vec!(MTest; 2);

    let actual: bool = id_vec_0 >= id_vec_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_vec_0 >= id_vec_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_vec_1 >= id_vec_0;
    let expected = true;
    assert_eq!(actual, expected);
}
