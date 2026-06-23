use crate::{
    UsizeId,
    tests::util::{BTest, extends::BTestBase},
    usize_id as id,
};

#[test]
fn downcast_to_test() {
    let usize_id = id!(BTestBase; 1);

    let actual: UsizeId<BTest> = usize_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let usize_id = id!(BTest; 1);

    let actual: UsizeId<BTestBase> = usize_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
