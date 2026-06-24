use crate::{
    U128Id,
    tests::util::{BTest, extends::BTestBase},
    u128_id as id,
};

#[test]
fn downcast_to_test() {
    let u128_id = id!(BTestBase; 1);

    let actual: U128Id<BTest> = u128_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let u128_id = id!(BTest; 1);

    let actual: U128Id<BTestBase> = u128_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
