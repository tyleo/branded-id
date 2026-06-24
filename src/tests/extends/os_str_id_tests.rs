use crate::{
    OsStrId, OsStringId, os_str_id, os_string_id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn borrowed_downcast_as_test() {
    let id = os_str_id!(BTestBase; "a");

    let actual: &OsStrId<BTest> = id.downcast_as();
    let expected = os_str_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_upcast_as_test() {
    let id = os_str_id!(BTest; "a");

    let actual: &OsStrId<BTestBase> = id.upcast_as();
    let expected = os_str_id!(BTestBase; "a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_downcast_as_test() {
    let id = os_string_id!(BTestBase; "a");

    let actual: &OsStringId<BTest> = id.downcast_as();
    let expected = os_string_id!(BTest; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_upcast_as_test() {
    let id = os_string_id!(BTest; "a");

    let actual: &OsStringId<BTestBase> = id.upcast_as();
    let expected = os_string_id!(BTestBase; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_downcast_as_mut_test() {
    let mut id = os_string_id!(BTestBase; "a");

    let actual: &mut OsStringId<BTest> = id.downcast_as_mut();
    let expected = os_string_id!(BTest; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_upcast_as_mut_test() {
    let mut id = os_string_id!(BTest; "a");

    let actual: &mut OsStringId<BTestBase> = id.upcast_as_mut();
    let expected = os_string_id!(BTestBase; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_downcast_into_test() {
    let id = os_string_id!(BTestBase; "a");

    let actual: OsStringId<BTest> = id.downcast_into();
    let expected = os_string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_upcast_into_test() {
    let id = os_string_id!(BTest; "a");

    let actual: OsStringId<BTestBase> = id.upcast_into();
    let expected = os_string_id!(BTestBase; "a");
    assert_eq!(actual, expected);
}
