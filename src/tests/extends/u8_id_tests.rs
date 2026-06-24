use crate::{
    U8Id,
    tests::util::{BTest, extends::BTestBase},
    u8_id as id,
};

#[test]
fn downcast_to_test() {
    let u8_id = id!(BTestBase; 1);

    let actual: U8Id<BTest> = u8_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let u8_id = id!(BTest; 1);

    let actual: U8Id<BTestBase> = u8_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
