use crate::{IdPtr, ext::PtrExt, id_ptr, tests::util::BTest};

#[test]
fn to_ptr_id_test() {
    let ptr: *const _ = &1;

    let actual: IdPtr<BTest, i32> = ptr.to_id_ptr();
    let expected = id_ptr!(ptr);
    assert_eq!(actual, expected);
}
