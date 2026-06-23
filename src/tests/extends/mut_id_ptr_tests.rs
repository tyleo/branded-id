use crate::{
    MutIdPtr, mut_id_ptr,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_to_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(BTestBase; ptr);

    let actual: MutIdPtr<BTest, i32> = id_ptr.downcast_to();
    let expected = mut_id_ptr!(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(BTest; ptr);

    let actual: MutIdPtr<BTestBase, i32> = id_ptr.upcast_to();
    let expected = mut_id_ptr!(ptr);
    assert_eq!(actual, expected);
}
