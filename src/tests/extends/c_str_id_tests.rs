use crate::{
    CStrId, CStringId, c_str_id, c_string_id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn borrowed_downcast_as_test() {
    let id = c_str_id!(BTestBase; c"a");

    let actual: &CStrId<BTest> = id.downcast_as();
    let expected = c_str_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_upcast_as_test() {
    let id = c_str_id!(BTest; c"a");

    let actual: &CStrId<BTestBase> = id.upcast_as();
    let expected = c_str_id!(BTestBase; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_downcast_as_test() {
    let id = c_string_id!(BTestBase; c"a");

    let actual: &CStringId<BTest> = id.downcast_as();
    let expected = c_string_id!(BTest; c"a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_upcast_as_test() {
    let id = c_string_id!(BTest; c"a");

    let actual: &CStringId<BTestBase> = id.upcast_as();
    let expected = c_string_id!(BTestBase; c"a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_downcast_as_mut_test() {
    let mut id = c_string_id!(BTestBase; c"a");

    let actual: &mut CStringId<BTest> = id.downcast_as_mut();
    let expected = c_string_id!(BTest; c"a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_upcast_as_mut_test() {
    let mut id = c_string_id!(BTest; c"a");

    let actual: &mut CStringId<BTestBase> = id.upcast_as_mut();
    let expected = c_string_id!(BTestBase; c"a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_downcast_into_test() {
    let id = c_string_id!(BTestBase; c"a");

    let actual: CStringId<BTest> = id.downcast_into();
    let expected = c_string_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_upcast_into_test() {
    let id = c_string_id!(BTest; c"a");

    let actual: CStringId<BTestBase> = id.upcast_into();
    let expected = c_string_id!(BTestBase; c"a");
    assert_eq!(actual, expected);
}
