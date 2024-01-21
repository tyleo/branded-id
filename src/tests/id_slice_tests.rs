use crate::{
    ext::{MutPtrExt, PtrExt, SliceExt},
    id_array, id_slice,
    tests::util::MTest,
    usize_id as id, IdPtr, IdSlice, MutIdPtr,
};

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
    let slice = &mut [1];

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

    let actual: crate::UsizeId<MTest> = id_slice.end();
    let expected = id!(1);
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
#[allow(clippy::redundant_slicing)]
fn index_test() {
    let id_slice = id_slice![MTest; 0, 1, 2, 3];

    let actual: i32 = id_slice[id!(1)];
    let expected = 1;
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &id_slice[id!(1)..id!(3)];
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &id_slice[id!(1)..];
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &id_slice[..];
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &id_slice[id!(1)..=id!(3)];
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &id_slice[..id!(3)];
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &id_slice[..=id!(3)];
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::redundant_slicing)]
fn index_mut_test() {
    let mut mut_array = [0, 1, 2, 3];
    let mut_id_slice = mut_array.as_mut_id_slice::<MTest>();

    let actual: i32 = mut_id_slice[id!(1)];
    let expected = 1;
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &mut_id_slice[id!(1)..id!(3)];
    let expected = id_slice![1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &mut_id_slice[id!(1)..];
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &mut_id_slice[..];
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &mut_id_slice[id!(1)..=id!(3)];
    let expected = id_slice![1, 2, 3];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &mut_id_slice[..id!(3)];
    let expected = id_slice![0, 1, 2];
    assert_eq!(actual, expected);

    let actual: &IdSlice<MTest, i32> = &mut_id_slice[..=id!(3)];
    let expected = id_slice![0, 1, 2, 3];
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_slice_0 = id_slice![MTest; 1];
    let id_slice_1 = id_slice![1];
    let id_slice_2 = id_slice![2];

    let actual = id_slice_0 == id_slice_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_slice_0 == id_slice_2;
    let expected = false;
    assert_eq!(actual, expected);
}
