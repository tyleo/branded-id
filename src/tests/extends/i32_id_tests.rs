use crate::{
    I32Id, i32_id as id,
    tests::util::{MTest, extends::MTestBase},
};

#[test]
fn downcast_to_test() {
    let i32_id = id!(MTestBase; 1);

    let actual: I32Id<MTest> = i32_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let i32_id = id!(MTest; 1);

    let actual: I32Id<MTestBase> = i32_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
