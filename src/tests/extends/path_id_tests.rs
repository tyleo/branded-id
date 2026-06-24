use crate::{
    PathBufId, PathId, path_buf_id, path_id,
    tests::util::{BTest, extends::BTestBase},
};

#[test]
fn borrowed_downcast_as_test() {
    let id = path_id!(BTestBase; "a");

    let actual: &PathId<BTest> = id.downcast_as();
    let expected = path_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_upcast_as_test() {
    let id = path_id!(BTest; "a");

    let actual: &PathId<BTestBase> = id.upcast_as();
    let expected = path_id!(BTestBase; "a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_downcast_as_test() {
    let id = path_buf_id!(BTestBase; "a");

    let actual: &PathBufId<BTest> = id.downcast_as();
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_upcast_as_test() {
    let id = path_buf_id!(BTest; "a");

    let actual: &PathBufId<BTestBase> = id.upcast_as();
    let expected = path_buf_id!(BTestBase; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_downcast_as_mut_test() {
    let mut id = path_buf_id!(BTestBase; "a");

    let actual: &mut PathBufId<BTest> = id.downcast_as_mut();
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_upcast_as_mut_test() {
    let mut id = path_buf_id!(BTest; "a");

    let actual: &mut PathBufId<BTestBase> = id.upcast_as_mut();
    let expected = path_buf_id!(BTestBase; "a");
    assert_eq!(*actual, expected);
}

#[test]
fn owned_downcast_into_test() {
    let id = path_buf_id!(BTestBase; "a");

    let actual: PathBufId<BTest> = id.downcast_into();
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_upcast_into_test() {
    let id = path_buf_id!(BTest; "a");

    let actual: PathBufId<BTestBase> = id.upcast_into();
    let expected = path_buf_id!(BTestBase; "a");
    assert_eq!(actual, expected);
}
