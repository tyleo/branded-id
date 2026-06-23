use crate::{
    IsizeId, isize_id as id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn downcast_to_test() {
    let isize_id = id!(BTestBase; 1);

    let actual: IsizeId<BTest> = isize_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let isize_id = id!(BTest; 1);

    let actual: IsizeId<BTestBase> = isize_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
