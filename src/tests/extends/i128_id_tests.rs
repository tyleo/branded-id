use crate::{
    I128Id, i128_id as id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_to_test() {
    let i128_id = id!(BTestBase; 1);

    let actual: I128Id<BTest> = i128_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let i128_id = id!(BTest; 1);

    let actual: I128Id<BTestBase> = i128_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
