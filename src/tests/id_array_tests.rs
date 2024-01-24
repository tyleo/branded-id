use crate::{id_array, id_slice, tests::util::MTest, usize_id as id, IdArray, IdSlice};
use std::{
    array::IntoIter,
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

#[test]
fn as_array_test() {
    let id_array = id_array![MTest; 1];

    let actual: &[i32; 1] = id_array.as_array();
    let expected = &[1];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_array_test() {
    let mut id_array = id_array![MTest; 1];

    let actual: &mut [i32; 1] = id_array.as_mut_array();
    actual[0] = 2;

    let expected = &[2];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_id_slice_test() {
    let id_array = &mut id_array![1];

    let actual: &mut IdSlice<MTest, i32> = id_array.as_mut_id_slice();
    actual[id!(0)] = 2;

    let expected = id_slice![MTest; i32; 2];
    assert_eq!(actual, expected);
}

#[test]
fn as_id_slice_test() {
    let id_array = &id_array![1];

    let actual: &IdSlice<MTest, i32> = id_array.as_id_slice();
    let expected = id_slice![MTest; i32; 1];
    assert_eq!(actual, expected);
}

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
fn into_array_test() {
    let id_array = id_array![MTest; 1];

    let actual: [i32; 1] = id_array.into_array();
    let expected = [1];
    assert_eq!(actual, expected);
}

#[test]
fn as_ref_test() {
    let id_array = id_array![1];

    let actual: &IdSlice<MTest, i32> = id_array.as_ref();
    let expected = id_slice![MTest; 1];
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_test() {
    let mut id_array = id_array![1];

    let actual: &mut IdSlice<MTest, i32> = id_array.as_mut();
    actual[id!(0)] = 2;

    let expected = id_slice![MTest; 2];
    assert_eq!(actual, expected);
}

#[test]
fn borrow_test() {
    let id_array = id_array![1];

    let actual: &IdSlice<MTest, i32> = id_array.borrow();
    let expected = id_slice![MTest; 1];
    assert_eq!(actual, expected);
}

#[test]
fn borrow_mut_test() {
    let mut id_array = id_array![1];

    let actual: &mut IdSlice<MTest, i32> = id_array.borrow_mut();
    actual[id!(0)] = 2;

    let expected = id_slice![MTest; 2];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let id_array = id_array!(MTest; 1);

    let actual: IdArray<MTest, i32, 1> = id_array.clone();
    let expected = id_array![MTest; i32; 1];
    assert_eq!(actual, expected);
}

#[test]
fn debug_test() {
    let empty_id_array = id_array![MTest; i32];
    let actual = format!("{:?}", empty_id_array);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:10?}", empty_id_array);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:>10?}", empty_id_array);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:^10?}", empty_id_array);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:.1?}", empty_id_array);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:#?}", empty_id_array);
    let expected = "id_sys::tests::util::m_test::MTest[]";
    assert_eq!(actual, expected);

    let array_0 = ["Hello"];
    let nested_id_array = id_array![MTest; array_0, ["Hi"]];

    let actual = format!("{:#?}", nested_id_array);
    let expected = "id_sys::tests::util::m_test::MTest[\n    [\n        \"Hello\",\n    ],\n    [\n        \"Hi\",\n    ],\n]";
    assert_eq!(actual, expected);

    let id_array = id_array![MTest; "Hello", "Hi"];

    let actual = format!("{:?}", id_array);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:10?}", id_array);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:>10?}", id_array);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:^10?}", id_array);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:.1?}", id_array);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:#?}", id_array);
    let expected = "id_sys::tests::util::m_test::MTest[\n    \"Hello\",\n    \"Hi\",\n]";
    assert_eq!(actual, expected);
}

#[test]
fn default_test() {
    let actual: IdArray<MTest, i32, 1> = <IdArray<MTest, i32, 1>>::default();
    let expected = id_array![MTest; i32; 0];
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
fn hash_test() {
    let id_array = id_array!(MTest; 1);
    let mut hasher_0 = DefaultHasher::new();
    id_array.hash(&mut hasher_0);

    let array = [1];
    let mut hasher_1 = DefaultHasher::new();
    array.hash(&mut hasher_1);

    let actual = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn hash_slice_test() {
    let id_arrays = [id_array!(MTest; 1), id_array!(MTest; 2)];
    let mut hasher_0 = DefaultHasher::new();
    IdArray::hash_slice(&id_arrays, &mut hasher_0);

    let arrays = [[1], [2]];
    let mut hasher_1 = DefaultHasher::new();
    <[i32; 1] as Hash>::hash_slice(&arrays, &mut hasher_1);

    let actual = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn index_test() {
    let id_array = id_array![MTest; 0, 1, 2, 3];

    let actual: &i32 = id_array.index(id!(1));
    let expected = &1;
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_array.index(id!(1)..id!(3));
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_array.index(id!(1)..);
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_array.index(..);
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_array.index(id!(1)..=id!(3));
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_array.index(..id!(3));
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_array.index(..=id!(3));
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn index_mut_test() {
    let mut id_array_mut = id_array![MTest; 0, 1, 2, 3];

    let actual: &mut i32 = id_array_mut.index_mut(id!(1));
    let expected = &1;
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_array_mut.index_mut(id!(1)..id!(3));
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_array_mut.index_mut(id!(1)..);
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_array_mut.index_mut(..);
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_array_mut.index_mut(id!(1)..=id!(3));
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_array_mut.index_mut(..id!(3));
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = id_array_mut.index_mut(..=id!(3));
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::into_iter_on_ref)]
fn into_iter_ref_test() {
    let id_array = &id_array![MTest; 0, 1, 2];

    let actual: Iter<'_, i32> = id_array.into_iter();
    let expected = (&[0, 1, 2]).into_iter();
    assert!(actual.eq(expected));
}

#[test]
#[allow(clippy::into_iter_on_ref)]
fn into_iter_mut_test() {
    let id_array = &mut id_array![MTest; 0, 1, 2];

    let mut expected = [0, 1, 2];

    let actual: IterMut<'_, i32> = id_array.into_iter();
    let expected = (&mut expected).into_iter();
    assert!(actual.eq(expected));
}

#[test]
fn into_iter_test() {
    let id_array = id_array![MTest; 0, 1, 2];

    let actual: IntoIter<i32, 3> = id_array.into_iter();
    let expected = [0, 1, 2].into_iter();
    assert!(actual.eq(expected));
}

#[test]
fn cmp_test() {
    let array_0 = id_array![MTest; 1];
    let array_1 = id_array![MTest; 2];

    let actual: Ordering = array_0.cmp(&array_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);
}

#[test]
fn max_test() {
    let array_0 = id_array![MTest; 1];
    let array_1 = id_array![MTest; 2];

    let actual: IdArray<MTest, i32, 1> = array_0.max(array_1);
    let expected = id_array![MTest; 2];
    assert_eq!(actual, expected);
}

#[test]
fn min_test() {
    let array_0 = id_array![MTest; 1];
    let array_1 = id_array![MTest; 2];

    let actual: IdArray<MTest, i32, 1> = array_0.min(array_1);
    let expected = id_array![MTest; 1];
    assert_eq!(actual, expected);
}

#[test]
fn clamp_test() {
    let array_0 = id_array![MTest; 1];
    let array_1 = id_array![MTest; 2];
    let array_2 = id_array![MTest; 3];

    let actual: IdArray<MTest, i32, 1> = array_2.clamp(array_0, array_1);
    let expected = id_array![MTest; 2];
    assert_eq!(actual, expected);

    let array_0 = id_array![MTest; 1];
    let array_1 = id_array![MTest; 2];
    let array_2 = id_array![MTest; 3];

    let actual: IdArray<MTest, i32, 1> = array_0.clamp(array_1, array_2);
    let expected = id_array![MTest; 2];
    assert_eq!(actual, expected);
}

#[test]
fn ref_slice_eq_test() {
    let id_array = id_array![MTest; 1];
    let id_slice_0 = id_slice![1];
    let id_slice_1 = id_slice![2];

    let actual = id_array.eq(&id_slice_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_array.eq(&id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ref_slice_ne_test() {
    let id_array = id_array![MTest; 1];
    let id_slice_0 = id_slice![1];
    let id_slice_1 = id_slice![2];

    let actual = id_array.ne(&id_slice_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_array.ne(&id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn mut_slice_eq_test() {
    let id_array = id_array![MTest; 1];
    let mut_slice_0 = &mut [1];
    let id_slice_0 = IdSlice::<MTest, _>::from_mut_slice(mut_slice_0);
    let mut_slice_1 = &mut [2];
    let id_slice_1 = IdSlice::<MTest, _>::from_mut_slice(mut_slice_1);

    let actual = id_array.eq(&id_slice_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_array.eq(&id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn mut_slice_ne_test() {
    let id_array = id_array![MTest; 1];
    let mut_slice_0 = &mut [1];
    let id_slice_0 = IdSlice::<MTest, _>::from_mut_slice(mut_slice_0);
    let mut_slice_1 = &mut [2];
    let id_slice_1 = IdSlice::<MTest, _>::from_mut_slice(mut_slice_1);

    let actual = id_array.ne(&id_slice_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_array.ne(&id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn slice_eq_test() {
    let id_array = id_array![MTest; 1];
    let id_slice_0 = id_slice![1];
    let id_slice_1 = id_slice![2];

    let actual = id_array.eq(id_slice_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_array.eq(id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn slice_ne_test() {
    let id_array = id_array![MTest; 1];
    let id_slice_0 = id_slice![1];
    let id_slice_1 = id_slice![2];

    let actual = id_array.ne(id_slice_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_array.ne(id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_array_0 = id_array![MTest; 1];
    let id_array_1 = id_array![1];
    let id_array_2 = id_array![2];

    let actual = id_array_0.eq(&id_array_1);
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_array_0.eq(&id_array_2);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_test() {
    let id_array_0 = id_array![MTest; 1];
    let id_array_1 = id_array![1];
    let id_array_2 = id_array![2];

    let actual = id_array_0.ne(&id_array_1);
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_array_0.ne(&id_array_2);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let id_array_0 = id_array![MTest; 1];
    let id_array_1 = id_array![MTest; 2];

    let actual: Option<Ordering> = id_array_0.partial_cmp(&id_array_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);
}

#[test]
fn lt_test() {
    let id_array_0 = id_array![MTest; 1];
    let id_array_1 = id_array![MTest; 2];

    let actual: bool = id_array_0.lt(&id_array_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn le_test() {
    let id_array_0 = id_array![MTest; 1];
    let id_array_1 = id_array![MTest; 2];

    let actual: bool = id_array_0.le(&id_array_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn gt_test() {
    let id_array_0 = id_array![MTest; 1];
    let id_array_1 = id_array![MTest; 2];

    let actual: bool = id_array_0.gt(&id_array_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ge_test() {
    let id_array_0 = id_array![MTest; 1];
    let id_array_1 = id_array![MTest; 2];

    let actual: bool = id_array_0.ge(&id_array_1);
    let expected = false;
    assert_eq!(actual, expected);
}
