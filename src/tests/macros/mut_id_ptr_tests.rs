use crate::{MutIdPtr, mut_id_ptr, tests::util::MTest};

#[test]
fn mut_id_ptr_0_test() {
    let mut value = 0;
    let ptr: *mut _ = &mut value;
    let actual: MutIdPtr<MTest, _> = mut_id_ptr!(ptr);
    let expected = MutIdPtr::from_mut_ptr(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_ptr_1_test() {
    let mut value = 0;
    let ptr: *mut _ = &mut value;
    let actual = mut_id_ptr!(MTest; ptr);
    let expected = MutIdPtr::from_mut_ptr(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn mut_id_ptr_2_test() {
    let mut value = 0;
    let ptr: *mut _ = &mut value;
    let actual = mut_id_ptr!(MTest; i32; ptr);
    let expected = MutIdPtr::from_mut_ptr(ptr);
    assert_eq!(actual, expected);
}
