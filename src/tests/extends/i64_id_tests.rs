use crate::{
    I64Id, i64_id as id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_to_test() {
    let i64_id = id!(BTestBase; 1);

    let actual: I64Id<BTest> = i64_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let i64_id = id!(BTest; 1);

    let actual: I64Id<BTestBase> = i64_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
