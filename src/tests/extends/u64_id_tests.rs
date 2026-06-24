use crate::{
    U64Id,
    tests::util::{BTest, extends::BTestBase},
    u64_id as id,
};

#[test]
fn downcast_to_test() {
    let u64_id = id!(BTestBase; 1);

    let actual: U64Id<BTest> = u64_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let u64_id = id!(BTest; 1);

    let actual: U64Id<BTestBase> = u64_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
