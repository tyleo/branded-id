use crate::{
    ext::{MutPtrExt, PtrExt, SliceExt},
    id_array, id_slice, id_vec,
    tests::util::MTest,
    usize_id as id, IdPtr, IdSlice, IdVec, MutIdPtr, UsizeId,
};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::{BufRead, IoSlice, IoSliceMut, Read, Write},
    net::{SocketAddr, ToSocketAddrs},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
    str::FromStr,
};

#[test]
fn as_mut_id_ptr_test() {
    let mut_slice = &mut [1];

    let mut_id_slice = mut_slice.as_mut_id_slice::<MTest>();

    let actual: MutIdPtr<MTest, i32> = mut_id_slice.as_mut_id_ptr();
    let expected = mut_slice.as_mut_ptr().to_mut_id_ptr::<MTest>();
    assert_eq!(actual, expected);
}

#[test]
fn as_mut_slice_test() {
    let mut_slice = &mut [1];
    let mut_id_slice = mut_slice.as_mut_id_slice::<MTest>();

    let actual: &mut [i32] = mut_id_slice.as_mut_slice();
    actual[0] = 2;

    let expected = &mut [2];
    assert_eq!(actual, expected);
}

#[test]
fn as_id_ptr_test() {
    let slice = &[1];

    let id_slice = slice.as_id_slice::<MTest>();

    let actual: IdPtr<MTest, i32> = id_slice.as_id_ptr();
    let expected = slice.as_ptr().to_id_ptr::<MTest>();
    assert_eq!(actual, expected);
}

#[test]
fn as_slice_test() {
    let id_slice = id_slice!(MTest; 1);

    let actual: &[i32] = id_slice.as_slice();
    let expected = &[1];
    assert_eq!(actual, expected);
}

#[test]
fn end_test() {
    let id_slice = id_slice![MTest; 1];

    let actual: UsizeId<MTest> = id_slice.end();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_slice() {
    let array = &mut [1];
    let mut_slice = array.as_mut_slice();

    let actual: &mut IdSlice<MTest, i32> = IdSlice::from_mut_slice(mut_slice);
    actual[id!(0)] = 2;

    let mut id_array = id_array![MTest; i32; 2];
    let expected = id_array.as_mut();

    assert_eq!(actual, expected);
}

#[test]
fn from_slice_test() {
    let slice = [1].as_slice();

    let actual: &IdSlice<MTest, i32> = IdSlice::from_slice(slice);
    let expected = id_slice![MTest; i32; 1];
    assert_eq!(actual, expected);
}

#[test]
fn is_empty_test() {
    let id_slice = id_slice![MTest; i32];

    let actual: bool = id_slice.is_empty();
    let expected = true;
    assert_eq!(actual, expected);

    let id_slice = id_slice![MTest; 1];

    let actual: bool = id_slice.is_empty();
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn iter_test() {
    let id_slice = id_slice![MTest; 0, 1, 2];

    let actual = id_slice.iter();
    let expected = [0, 1, 2].as_slice().iter();
    assert!(actual.eq(expected));
}

#[test]
fn iter_mut_test() {
    let mut array = id_array![MTest; 0, 1, 2];
    let mut_slice = array.as_mut_id_slice();

    let actual = mut_slice.iter_mut();
    let expected = [0, 1, 2].as_slice().iter();
    assert!(actual.eq(expected));
}

#[test]
fn len_test() {
    let id_slice = id_slice![MTest; i32];

    let actual: usize = id_slice.len();
    let expected = 0;
    assert_eq!(actual, expected);

    let id_slice = id_slice![MTest; 1];

    let actual: usize = id_slice.len();
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::useless_asref)]
fn as_mut_test() {
    let mut_slice = &mut [1];
    let mut_id_slice = mut_slice.as_mut_id_slice::<MTest>();

    let actual: &mut IdSlice<MTest, i32> = mut_id_slice.as_mut();
    actual[id!(0)] = 2;

    let mut expected = id_array![MTest; 2];
    let expected = expected.as_mut();
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::useless_asref)]
fn as_ref_test() {
    let id_slice = id_slice![MTest; 1];

    let actual: &IdSlice<MTest, i32> = id_slice.as_ref();
    let expected = id_slice![MTest; 1];
    assert_eq!(actual, expected);
}

#[test]
fn fill_buf_test() {
    let id_slice = &mut id_slice![MTest; 1, 2];

    let actual: &[u8] = id_slice.fill_buf().unwrap();
    let expected = &[1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn consume_test() {
    let id_slice = &mut id_slice![MTest; 1, 2];

    id_slice.consume(2);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn read_until_test() {
    let id_slice = &mut id_slice![MTest; 1, 2, 3];

    let mut buf = vec![];

    let actual: usize = id_slice.read_until(2, &mut buf).unwrap();
    let expected = 2;
    assert_eq!(actual, expected);

    let actual: &Vec<u8> = &buf;
    let expected = &[1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![3];
    assert_eq!(actual, expected);
}

#[test]
fn read_line_test() {
    let id_slice = &mut id_slice![MTest; 0xA];

    let mut buf = String::new();

    let actual: usize = id_slice.read_line(&mut buf).unwrap();
    let expected = 1;
    assert_eq!(actual, expected);

    let actual: &String = &buf;
    let expected = &String::from_str("\n").unwrap();
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn debug_test() {
    let empty_id_slice = id_slice![MTest; i32];
    let actual = format!("{:?}", empty_id_slice);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:10?}", empty_id_slice);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:>10?}", empty_id_slice);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:^10?}", empty_id_slice);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:.1?}", empty_id_slice);
    let expected = "MTest[]";
    assert_eq!(actual, expected);

    let actual = format!("{:#?}", empty_id_slice);
    let expected = "id_sys::tests::util::m_test::MTest[]";
    assert_eq!(actual, expected);

    let nested_id_slice = IdSlice::<MTest, _>::from_slice(&[["Hello"], ["Hi"]]);

    let actual = format!("{:#?}", nested_id_slice);
    let expected = "id_sys::tests::util::m_test::MTest[\n    [\n        \"Hello\",\n    ],\n    [\n        \"Hi\",\n    ],\n]";
    assert_eq!(actual, expected);

    let id_slice = id_slice![MTest; "Hello", "Hi"];

    let actual = format!("{:?}", id_slice);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:10?}", id_slice);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:>10?}", id_slice);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:^10?}", id_slice);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:.1?}", id_slice);
    let expected = "MTest[\"Hello\", \"Hi\"]";
    assert_eq!(actual, expected);

    let actual = format!("{:#?}", id_slice);
    let expected = "id_sys::tests::util::m_test::MTest[\n    \"Hello\",\n    \"Hi\",\n]";
    assert_eq!(actual, expected);
}

#[test]
fn default_test() {
    let actual: &IdSlice<MTest, i32> = <&IdSlice<MTest, i32>>::default();
    let expected = id_slice![MTest; i32];
    assert_eq!(actual, expected);
}

#[test]
fn default_mut_test() {
    let actual: &mut IdSlice<MTest, i32> = <&mut IdSlice<MTest, i32>>::default();
    let expected = id_slice![MTest; i32];
    assert_eq!(actual, expected);
}

#[test]
fn slice_from_slice_test() {
    let slice: &[i32] = &[0];

    let actual: &IdSlice<MTest, i32> = From::from(slice);
    let expected = id_slice!(MTest; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn slice_from_mut_slice_test() {
    let slice: &mut [i32] = &mut [0];

    let actual: &IdSlice<MTest, i32> = From::from(slice);
    let expected = id_slice!(MTest; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn mut_slice_from_mut_test() {
    let slice: &mut [i32] = &mut [0];

    let actual: &mut IdSlice<MTest, i32> = From::from(slice);
    let expected = id_slice!(MTest; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn hash_test() {
    let id_slice = id_slice!(MTest; 1);
    let mut hasher_0 = DefaultHasher::new();
    id_slice.hash(&mut hasher_0);

    let slice = [1].as_slice();
    let mut hasher_1 = DefaultHasher::new();
    slice.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::redundant_slicing)]
fn index_test() {
    let id_slice = id_slice![MTest; 0, 1, 2, 3];

    let actual: &i32 = id_slice.index(id!(1));
    let expected = &1;
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_slice.index(id!(1)..id!(3));
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_slice.index(id!(1)..);
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_slice.index(..);
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_slice.index(id!(1)..=id!(3));
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_slice.index(..id!(3));
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = id_slice.index(..=id!(3));
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::redundant_slicing)]
fn index_mut_test() {
    let mut mut_array = [0, 1, 2, 3];
    let mut_id_slice = mut_array.as_mut_id_slice::<MTest>();

    let actual: &mut i32 = mut_id_slice.index_mut(id!(1));
    let expected = &1;
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = mut_id_slice.index_mut(id!(1)..id!(3));
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = mut_id_slice.index_mut(id!(1)..);
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = mut_id_slice.index_mut(..);
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = mut_id_slice.index_mut(id!(1)..=id!(3));
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = mut_id_slice.index_mut(..id!(3));
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);

    let actual: &mut IdSlice<MTest, i32> = mut_id_slice.index_mut(..=id!(3));
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::into_iter_on_ref)]
fn into_iter_test() {
    let id_slice = id_slice![MTest; 0, 1, 2];

    let actual: Iter<'_, i32> = id_slice.into_iter();
    let expected = [0, 1, 2].as_slice().into_iter();
    assert!(actual.eq(expected));
}

#[test]
#[allow(clippy::into_iter_on_ref)]
fn into_iter_mut_test() {
    let mut array = id_array![MTest; 0, 1, 2];
    let mut_slice = array.as_mut_id_slice();

    let actual: IterMut<'_, i32> = mut_slice.into_iter();
    let expected = [0, 1, 2].as_slice().into_iter();
    assert!(actual.eq(expected));
}

#[test]
fn cmp_test() {
    let slice_0 = id_slice![MTest; 1];
    let slice_1 = id_slice![MTest; 2];

    let actual: Ordering = slice_0.cmp(slice_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_slice_1 = id_slice![1];
    let id_slice_2 = id_slice![2];

    let actual: bool = id_slice_0.eq(id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_slice_0.eq(id_slice_2);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_slice_1 = id_slice![1];
    let id_slice_2 = id_slice![2];

    let actual: bool = id_slice_0.ne(id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_slice_0.ne(id_slice_2);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn eq_array_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_array_1 = id_array![1];
    let id_array_2 = id_array![2];

    let actual: bool = id_slice_0.eq(&id_array_1);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_slice_0.eq(&id_array_2);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_array_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_array_1 = id_array![1];
    let id_array_2 = id_array![2];

    let actual: bool = id_slice_0.ne(&id_array_1);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_slice_0.ne(&id_array_2);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn eq_vec_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_vec_1 = id_vec![1];
    let id_vec_2 = id_vec![2];

    let actual: bool = id_slice_0.eq(&id_vec_1);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_slice_0.eq(&id_vec_2);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_vec_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_vec_1 = id_vec![1];
    let id_vec_2 = id_vec![2];

    let actual: bool = id_slice_0.ne(&id_vec_1);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_slice_0.ne(&id_vec_2);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_slice_1 = id_slice![MTest; 2];

    let actual: Option<Ordering> = id_slice_0.partial_cmp(id_slice_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);
}

#[test]
fn lt_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_slice_1 = id_slice![MTest; 2];

    let actual: bool = id_slice_0.lt(id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn le_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_slice_1 = id_slice![MTest; 2];

    let actual: bool = id_slice_0.le(id_slice_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn gt_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_slice_1 = id_slice![MTest; 2];

    let actual: bool = id_slice_0.gt(id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ge_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_slice_1 = id_slice![MTest; 2];

    let actual: bool = id_slice_0.ge(id_slice_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn read_test() {
    let id_slice = &mut id_slice![MTest; 1, 2];

    let buf = &mut [0, 0];

    let actual: usize = id_slice.read(buf).unwrap();
    let expected = 2;
    assert_eq!(actual, expected);

    let actual: &[u8] = buf;
    let expected = &[1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn read_vectored_test() {
    let id_slice = &mut id_slice![MTest; 1, 2];

    let mut buf_slice = [0, 0];
    let buf = &mut [IoSliceMut::new(&mut buf_slice)];

    let actual: usize = id_slice.read_vectored(buf).unwrap();
    let expected = 2;
    assert_eq!(actual, expected);

    let actual: &[u8] = &buf_slice;
    let expected = &[1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn read_exact_test() {
    let id_slice = &mut id_slice![MTest; 1, 2];

    let buf = &mut [0, 0];

    id_slice.read_exact(buf).unwrap();

    let actual: &[u8] = buf;
    let expected = &[1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn read_to_end_test() {
    let id_slice = &mut id_slice![MTest; 1, 2];

    let buf = &mut vec![];

    let actual: usize = id_slice.read_to_end(buf).unwrap();
    let expected = 2;
    assert_eq!(actual, expected);

    let actual: &Vec<u8> = buf;
    let expected = &[1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn read_to_string_test() {
    let id_slice = &mut id_slice![MTest; 0];

    let buf = &mut String::new();

    let actual: usize = id_slice.read_to_string(buf).unwrap();
    let expected = 1;
    assert_eq!(actual, expected);

    let actual: &mut String = buf;
    let expected = &String::from_str("\0").unwrap();
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);
}

#[test]
fn to_owned_test() {
    let id_slice = &mut id_slice![MTest; 1];

    let actual: IdVec<MTest, i32> = id_slice.to_owned();
    let expected = id_vec![MTest; 1];
    assert_eq!(actual, expected);
}

#[test]
fn clone_into_test() {
    let slice = &mut [1];
    let mut_id_slice = slice.as_mut_id_slice::<MTest>();

    let actual: &mut IdVec<MTest, i32> = &mut id_vec![];
    mut_id_slice.clone_into(actual);

    let expected = &id_vec![MTest; 1];
    assert_eq!(actual, expected);
}

#[test]
fn to_socket_addrs_test() {
    let id_slice = id_slice![MTest; SocketAddr];

    let actual = id_slice.to_socket_addrs().unwrap();
    let expected = [];
    assert!(actual.eq(expected));
}

#[test]
fn write_test() {
    let mut_slice = &mut [0, 0];
    let mut_id_slice = &mut mut_slice.as_mut_id_slice::<MTest>();

    let buf = &[1, 2];

    let actual: usize = mut_id_slice.write(buf).unwrap();
    let expected = 2;
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, u8> = mut_id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);

    let actual: &[u8] = mut_slice;
    let expected = &[1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn write_vectored_test() {
    let slice = &mut [0, 0];
    let mut_id_slice = &mut slice.as_mut_id_slice::<MTest>();

    let bufs = &[IoSlice::new(&[1, 2])];

    let actual: usize = mut_id_slice.write_vectored(bufs).unwrap();
    let expected = 2;
    assert_eq!(actual, expected);

    let actual: &[u8] = slice;
    let expected = &[1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn write_all_test() {
    let mut_slice = &mut [0, 0];
    let mut_id_slice = &mut mut_slice.as_mut_id_slice::<MTest>();

    let buf = &[1, 2];

    mut_id_slice.write_all(buf).unwrap();

    let actual: &IdSlice<MTest, u8> = mut_id_slice;
    let expected = id_slice![];
    assert_eq!(actual, expected);

    let actual: &[u8] = mut_slice;
    let expected = &[1, 2];
    assert_eq!(actual, expected);
}

#[test]
fn flush_test() {
    let mut_slice = &mut [0, 0];
    let mut_id_slice = &mut mut_slice.as_mut_id_slice::<MTest>();

    mut_id_slice.flush().unwrap();
}

#[test]
fn write_fmt() {
    let mut_slice = &mut [1, 2];
    let mut_id_slice = &mut mut_slice.as_mut_id_slice::<MTest>();

    let arguments = format_args!("\0");

    mut_id_slice.write_fmt(arguments).unwrap();

    let actual: &IdSlice<MTest, u8> = mut_id_slice;
    let expected = id_slice![2];
    assert_eq!(actual, expected);

    let actual: &[u8] = mut_slice;
    let expected = &[0, 2];
    assert_eq!(actual, expected);
}
