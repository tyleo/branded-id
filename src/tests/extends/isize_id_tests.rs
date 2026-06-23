use crate::{
    IsizeId, isize_id as id,
    tests::util::{MTest, extends::MTestBase},
};

#[test]
fn downcast_to_test() {
    let isize_id = id!(MTestBase; 1);

    let actual: IsizeId<MTest> = isize_id.downcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn upcast_to_test() {
    let isize_id = id!(MTest; 1);

    let actual: IsizeId<MTestBase> = isize_id.upcast_to();
    let expected = id!(1);
    assert_eq!(actual, expected);
}
