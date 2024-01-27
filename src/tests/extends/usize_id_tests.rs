use crate::{
    tests::util::{extends::MTestBase, MTest},
    usize_id as id, UsizeId,
};

#[test]
fn downcast_to_test() {
    let usize_id = id!(MTestBase; 1);

    let actual: UsizeId<MTest> = usize_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let usize_id = id!(MTest; 1);

    let actual: UsizeId<MTestBase> = usize_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
