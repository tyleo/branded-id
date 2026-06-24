use crate::{
    I8Id, i8_id as id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_to_test() {
    let i8_id = id!(BTestBase; 1);

    let actual: I8Id<BTest> = i8_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let i8_id = id!(BTest; 1);

    let actual: I8Id<BTestBase> = i8_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
