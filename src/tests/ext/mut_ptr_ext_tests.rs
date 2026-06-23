use crate::{MutIdPtr, ext::MutPtrExt, mut_id_ptr, tests::util::BTest};

#[test]
fn to_ptr_id_test() {
    let ptr: *mut _ = &mut 1;

    let actual: MutIdPtr<BTest, i32> = ptr.to_mut_id_ptr();
    let expected = mut_id_ptr!(ptr);
    assert_eq!(actual, expected);
}
