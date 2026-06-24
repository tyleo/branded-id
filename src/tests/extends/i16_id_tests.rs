use crate::{
    I16Id, i16_id as id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_to_test() {
    let i16_id = id!(BTestBase; 1);

    let actual: I16Id<BTest> = i16_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let i16_id = id!(BTest; 1);

    let actual: I16Id<BTestBase> = i16_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
