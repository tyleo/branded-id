use crate::{
    I32Id, i32_id as id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_to_test() {
    let i32_id = id!(BTestBase; 1);

    let actual: I32Id<BTest> = i32_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let i32_id = id!(BTest; 1);

    let actual: I32Id<BTestBase> = i32_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
