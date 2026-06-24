use crate::{
    U16Id,
    tests::util::{BTest, extends::BTestBase},
    u16_id as id,
};

#[test]
fn downcast_to_test() {
    let u16_id = id!(BTestBase; 1);

    let actual: U16Id<BTest> = u16_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let u16_id = id!(BTest; 1);

    let actual: U16Id<BTestBase> = u16_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
