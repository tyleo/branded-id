use crate::{id_ptr, tests::util::MTest, IdPtr};

#[test]
fn id_ptr_0_test() {
    let ptr: *const _ = &0;
    let actual: IdPtr<MTest, _> = id_ptr!(ptr);
    let expected = IdPtr::from_ptr(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn id_ptr_1_test() {
    let ptr: *const _ = &0;
    let actual = id_ptr!(MTest; ptr);
    let expected = IdPtr::from_ptr(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn id_ptr_2_test() {
    let ptr: *const _ = &0;
    let actual = id_ptr!(MTest; i32; ptr);
    let expected = IdPtr::from_ptr(ptr);
    assert_eq!(actual, expected);
}
