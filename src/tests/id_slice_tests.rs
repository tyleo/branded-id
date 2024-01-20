use crate::{
    id::{tests::marker::MUnknown, GetIdSlice, GetIdSliceMut, Id32, Id32I, IdSlice},
    id32, id32i, id_slice, mut_id_slice,
};

#[test]
fn from_slice_test() {
    let array = [1];
    let id_slice = IdSlice::from_slice(&array);

    let id = id32!(MUnknown; 0);

    let actual = id_slice[id];
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn from_slice_mut_test() {
    let mut array = [1];
    let id_slice = IdSlice::from_slice_mut(&mut array);

    let id = id32!(MUnknown; 0);
    id_slice[id] = 2;

    let actual = id_slice[id];
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn end_test() {
    let id_slice = id_slice![MUnknown; 1];

    let actual = id_slice.end();
    let expected = id32!(1);
    assert_eq!(actual, expected);
}

#[test]
fn index_signed_test() {
    unsafe {
        let id = id32i!(-1);

        let id_slice = id_slice![MUnknown; 1, 2, 3];
        let offset = id_slice.offset_signed(1);

        let actual = *offset.index_signed(id);
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

#[test]
fn index_signed_mut_test() {
    unsafe {
        let id = id32i!(-1);

        let id_slice = mut_id_slice![MUnknown; 1, 2, 3];
        let offset = id_slice.offset_signed_mut(1);

        *offset.index_signed_mut(id) = 4;

        let actual = *id_slice.index_signed(id);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}

#[test]
fn is_empty_test() {
    let id_slice = id_slice![MUnknown; i32];

    let actual = id_slice.is_empty();
    let expected = true;
    assert_eq!(actual, expected);

    let id_slice = id_slice![MUnknown; 1];

    let actual = id_slice.is_empty();
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn len_test() {
    let id_slice = id_slice![MUnknown; i32];

    let actual = id_slice.len();
    let expected = 0;
    assert_eq!(actual, expected);

    let id_slice = id_slice![MUnknown; 1];

    let actual = id_slice.len();
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn offset_test() {
    unsafe {
        let id = id32!(0);

        let id_slice = id_slice![MUnknown; 1, 2, 3];
        let offset = id_slice.offset(1);

        let actual = offset[id];
        let expected = 2;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 2;
        assert_eq!(actual, expected);

        let offset = offset.offset(1);

        let actual = offset[id];
        let expected = 3;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

#[test]
fn offset_mut_test() {
    unsafe {
        let id = id32!(0);

        let id_slice = mut_id_slice![MUnknown; 1, 2, 3];
        let offset = id_slice.offset_mut(1);

        offset[id] = 4;
        let actual = offset[id];
        let expected = 4;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 2;
        assert_eq!(actual, expected);

        let offset = offset.offset_mut(1);

        offset[id] = 4;
        let actual = offset[id];
        let expected = 4;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

#[test]
fn offset_signed_test() {
    unsafe {
        let id = id32!(0);

        let id_slice = id_slice![MUnknown; 1, 2, 3];
        let offset = id_slice.offset_signed(1);

        let actual = offset[id];
        let expected = 2;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 2;
        assert_eq!(actual, expected);

        let offset = offset.offset_signed(1);

        let actual = offset[id];
        let expected = 3;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 1;
        assert_eq!(actual, expected);

        let offset = offset.offset_signed(-2);

        let actual = offset[id];
        let expected = 1;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 3;
        assert_eq!(actual, expected);
    }
}

#[test]
fn offset_signed_mut_test() {
    unsafe {
        let id = id32!(0);

        let id_slice = mut_id_slice![MUnknown; 1, 2, 3];
        let offset = id_slice.offset_signed_mut(1);

        offset[id] = 4;
        let actual = offset[id];
        let expected = 4;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 2;
        assert_eq!(actual, expected);

        let offset = offset.offset_signed_mut(1);

        offset[id] = 4;
        let actual = offset[id];
        let expected = 4;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 1;
        assert_eq!(actual, expected);

        let offset = offset.offset_signed_mut(-2);

        offset[id] = 4;
        let actual = offset[id];
        let expected = 4;
        assert_eq!(actual, expected);

        let actual = offset.len();
        let expected = 3;
        assert_eq!(actual, expected);
    }
}

#[test]
fn get_id_slice_test() {
    let id_slice_0 = mut_id_slice![MUnknown; 1];
    let id_slice_1 = id_slice_0.get_id_slice();

    let id = id32!(0);

    let actual = id_slice_1[id];
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn get_id_slice_mut_test() {
    let id_slice_0 = mut_id_slice![MUnknown; 1];
    let id_slice_1 = id_slice_0.get_id_slice_mut();

    let id = id32!(0);
    id_slice_1[id] = 2;

    let actual = id_slice_1[id];
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn from_array_test() {
    let array = [0];

    let actual: &IdSlice<MUnknown, i32> = From::from(array);
    let expected = id_slice!(MUnknown; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn from_array_mut_test() {
    let array = [0];

    let actual: &mut IdSlice<MUnknown, i32> = From::from(array);
    let expected = id_slice!(MUnknown; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn from_array_ref_test() {
    let array = &[0];

    let actual: &IdSlice<MUnknown, i32> = From::from(array);
    let expected = id_slice!(MUnknown; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_array_ref_test() {
    let array = &mut [0];

    let actual: &IdSlice<MUnknown, i32> = From::from(array);
    let expected = id_slice!(MUnknown; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_array_ref_mut_test() {
    let array = &mut [0];

    let actual: &mut IdSlice<MUnknown, i32> = From::from(array);
    let expected = id_slice!(MUnknown; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn from_from_slice_test() {
    let slice: &[i32] = &[0];

    let actual: &IdSlice<MUnknown, i32> = From::from(slice);
    let expected = id_slice!(MUnknown; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_slice_test() {
    let slice: &mut [i32] = &mut [0];

    let actual: &IdSlice<MUnknown, i32> = From::from(slice);
    let expected = id_slice!(MUnknown; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_slice_mut_test() {
    let slice: &mut [i32] = &mut [0];

    let actual: &mut IdSlice<MUnknown, i32> = From::from(slice);
    let expected = id_slice!(MUnknown; i32; 0);
    assert_eq!(actual, expected);
}

#[test]
fn index_test() {
    let id_slice = id_slice![MUnknown; 1];

    let id = id32!(0);

    let actual = id_slice[id];
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn index_mut_test() {
    let id_slice = mut_id_slice![MUnknown; 1];

    let id = id32!(0);
    id_slice[id] = 2;

    let actual = id_slice[id];
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_slice_0 = id_slice![MUnknown; 1];
    let id_slice_1 = id_slice![1];

    let actual = id_slice_0 == id_slice_1;
    let expected = true;
    assert_eq!(actual, expected);
}
