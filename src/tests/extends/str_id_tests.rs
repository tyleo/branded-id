use crate::{
    StrId, StringId, str_id, string_id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn borrowed_downcast_as_test() {
    let id = str_id!(BTestBase; "a");

    let actual: &StrId<BTest> = id.downcast_as();
    let expected = str_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_upcast_as_test() {
    let id = str_id!(BTest; "a");

    let actual: &StrId<BTestBase> = id.upcast_as();
    let expected = str_id!(BTestBase; "a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_downcast_as_test() {
    let id = string_id!(BTestBase; "a");

    let actual: &StringId<BTest> = id.downcast_as();
    let expected = string_id!(BTest; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_upcast_as_test() {
    let id = string_id!(BTest; "a");

    let actual: &StringId<BTestBase> = id.upcast_as();
    let expected = string_id!(BTestBase; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_downcast_as_mut_test() {
    let mut id = string_id!(BTestBase; "a");

    let actual: &mut StringId<BTest> = id.downcast_as_mut();
    let expected = string_id!(BTest; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_upcast_as_mut_test() {
    let mut id = string_id!(BTest; "a");

    let actual: &mut StringId<BTestBase> = id.upcast_as_mut();
    let expected = string_id!(BTestBase; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_downcast_into_test() {
    let id = string_id!(BTestBase; "a");

    let actual: StringId<BTest> = id.downcast_into();
    let expected = string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_upcast_into_test() {
    let id = string_id!(BTest; "a");

    let actual: StringId<BTestBase> = id.upcast_into();
    let expected = string_id!(BTestBase; "a");
    assert_eq!(actual, expected);
}
