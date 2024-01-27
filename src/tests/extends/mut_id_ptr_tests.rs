use crate::{
    mut_id_ptr,
    tests::util::{extends::MTestBase, MTest},
    MutIdPtr,
};

#[test]
fn downcast_to_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(MTestBase; ptr);

    let actual: MutIdPtr<MTest, i32> = id_ptr.downcast_to();
    let expected = mut_id_ptr!(ptr);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(MTest; ptr);

    let actual: MutIdPtr<MTestBase, i32> = id_ptr.upcast_to();
    let expected = mut_id_ptr!(ptr);
    assert_eq!(actual, expected);
}
