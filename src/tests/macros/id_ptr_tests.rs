use crate::{IdPtr, id_ptr, tests::util::BTest};

#[test]
fn id_ptr_0_test() {
    let ptr: *const _ = &0;
    let actual: IdPtr<BTest, _> = id_ptr!(ptr);
    let expected = IdPtr::from_ptr(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn id_ptr_1_test() {
    let ptr: *const _ = &0;
    let actual = id_ptr!(BTest; ptr);
    let expected = IdPtr::from_ptr(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn id_ptr_2_test() {
    let ptr: *const _ = &0;
    let actual = id_ptr!(BTest; i32; ptr);
    let expected = IdPtr::from_ptr(ptr);
    assert_eq!(actual, expected);
}
