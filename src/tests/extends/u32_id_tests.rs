use crate::{
    U32Id,
    tests::util::{BTest, extends::BTestBase},
    u32_id as id,
};

#[test]
fn downcast_to_test() {
    let u32_id = id!(BTestBase; 1);

    let actual: U32Id<BTest> = u32_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let u32_id = id!(BTest; 1);

    let actual: U32Id<BTestBase> = u32_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
