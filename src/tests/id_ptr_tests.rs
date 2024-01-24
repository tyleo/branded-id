use crate::{id_ptr, isize_id, tests::util::MTest, IdPtr};

#[test]
fn deref_ptr_test() {
    unsafe {
        let ptr: *const _ = &1;
        let id_ptr = id_ptr!(MTest; ptr);

        let actual = id_ptr.deref_ptr();
        let expected = &*ptr;
        assert_eq!(actual, expected);
    }
}

#[test]
fn from_ptr_test() {
    let ptr: *const _ = &1;

    let actual: IdPtr<MTest, i32> = IdPtr::from_ptr(ptr);
    let expected = id_ptr!(MTest; &1i32);
    assert_eq!(actual, expected);
}

#[test]
fn offset_test() {
    unsafe {
        let slice = [1, 2].as_slice();

        let id_ptr = id_ptr!(MTest; slice.as_ptr());
        let id_ptr = id_ptr.offset(isize_id!(1));

        let ptr = slice.as_ptr();
        let ptr = ptr.offset(1);

        let actual = id_ptr.deref_ptr();
        let expected = &*ptr;
        assert_eq!(actual, expected);
    }
}

#[test]
fn to_ptr_test() {
    let ptr: *const _ = &1;
    let id_ptr = id_ptr!(MTest; ptr);

    let actual = id_ptr.to_ptr();
    let expected = ptr;
    assert_eq!(actual, expected);
}

#[test]
fn from_const_ptr_test() {
    let ptr: *const _ = &1;

    let actual: IdPtr<MTest, i32> = IdPtr::from(ptr);
    let expected = id_ptr!(MTest; &1i32);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_ptr_test() {
    let ptr: *mut _ = &mut 1;

    let actual: IdPtr<MTest, i32> = IdPtr::from(ptr);
    let expected = id_ptr!(MTest; ptr);
    assert_eq!(actual, expected);
}

#[test]
fn id_ptr_test() {
    let ptr: *const _ = &1;
    let id_ptr_0 = id_ptr!(MTest; ptr);
    let id_ptr_1 = id_ptr!(MTest; ptr);

    let actual = id_ptr_0 == id_ptr_1;
    let expected = true;
    assert_eq!(actual, expected);
}
