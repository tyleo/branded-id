use crate::{
    IdPtr, id_ptr,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_to_test() {
    let ptr: *const _ = &1;
    let id_ptr = id_ptr!(BTestBase; ptr);

    let actual: IdPtr<BTest, i32> = id_ptr.downcast_to();
    let expected = id_ptr!(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let ptr: *const _ = &1;
    let id_ptr = id_ptr!(BTest; ptr);

    let actual: IdPtr<BTestBase, i32> = id_ptr.upcast_to();
    let expected = id_ptr!(ptr);
    assert_eq!(actual, expected);
}
