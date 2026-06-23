use crate::{
    U32Id,
    tests::util::{MTest, extends::MTestBase},
    u32_id as id,
};

#[test]
fn downcast_to_test() {
    let u32_id = id!(MTestBase; 1);

    let actual: U32Id<MTest> = u32_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let u32_id = id!(MTest; 1);

    let actual: U32Id<MTestBase> = u32_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
