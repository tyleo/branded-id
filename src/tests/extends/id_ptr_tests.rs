use crate::{
    IdPtr, id_ptr,
    tests::util::{MTest, extends::MTestBase},
};

#[test]
fn downcast_to_test() {
    let ptr: *const _ = &1;
    let id_ptr = id_ptr!(MTestBase; ptr);

    let actual: IdPtr<MTest, i32> = id_ptr.downcast_to();
    let expected = id_ptr!(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let ptr: *const _ = &1;
    let id_ptr = id_ptr!(MTest; ptr);

    let actual: IdPtr<MTestBase, i32> = id_ptr.upcast_to();
    let expected = id_ptr!(ptr);
    assert_eq!(actual, expected);
}
