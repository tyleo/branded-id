use crate::{PathBufId, PathId, path_buf_id, path_id, tests::util::BTest};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::{HashMap, hash_map::DefaultHasher},
    ffi::OsString,
    hash::{Hash, Hasher},
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

#[test]
fn from_path_test() {
    let actual: &PathId<BTest> = PathId::<BTest>::from_path(Path::new("a"));
    let expected = path_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn as_path_test() {
    let id = path_id!(BTest; "a");

    let actual: &Path = id.as_path();
    let expected = Path::new("a");
    assert_eq!(actual, expected);
}

#[test]
fn from_path_buf_test() {
    let actual: PathBufId<BTest> = PathBufId::<BTest>::from_path_buf(PathBuf::from("a"));
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn into_path_buf_test() {
    let id = path_buf_id!(BTest; "a");

    let actual: PathBuf = id.into_path_buf();
    let expected = PathBuf::from("a");
    assert_eq!(actual, expected);
}

#[test]
fn from_path_ref_test() {
    let actual: &PathId<BTest> = From::from(Path::new("a"));
    let expected = path_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_path_buf_value_test() {
    let actual: PathBufId<BTest> = From::from(PathBuf::from("a"));
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_str_slice_test() {
    let actual: PathBufId<BTest> = From::from("a");
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_string_test() {
    let actual: PathBufId<BTest> = From::from(String::from("a"));
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_os_string_test() {
    let actual: PathBufId<BTest> = From::from(OsString::from("a"));
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_path_owned_test() {
    let actual: PathBufId<BTest> = From::from(Path::new("a"));
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_str_trait_test() {
    let actual: PathBufId<BTest> = <PathBufId<BTest> as FromStr>::from_str("a").unwrap();
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_debug_fmt_test() {
    let id = path_id!(BTest; "a");

    let actual: String = format!("{:?}", id);
    let expected = "BTest(\"a\")";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = "branded_id::tests::util::b_test::BTest(\"a\")";
    assert_eq!(actual, expected);
}

#[test]
fn owned_debug_fmt_test() {
    let id = path_buf_id!(BTest; "a");

    let actual: String = format!("{:?}", id);
    let expected = "BTest(\"a\")";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = "branded_id::tests::util::b_test::BTest(\"a\")";
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let id = path_buf_id!(BTest; "a");

    let actual: PathBufId<BTest> = id.clone();
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_eq_test() {
    let id_0 = path_id!(BTest; "a");
    let id_1 = path_id!(BTest; "b");

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn owned_eq_test() {
    let id_0 = path_buf_id!(BTest; "a");
    let id_1 = path_buf_id!(BTest; "b");

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_cmp_test() {
    let id_0 = path_id!(BTest; "a");
    let id_1 = path_id!(BTest; "b");

    let actual: Ordering = id_0.cmp(id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_0.partial_cmp(id_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);
}

#[test]
fn owned_cmp_test() {
    let id_0 = path_buf_id!(BTest; "a");
    let id_1 = path_buf_id!(BTest; "b");

    let actual: Ordering = id_0.cmp(&id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_hash_matches_owned_test() {
    let borrowed = path_id!(BTest; "a");
    let mut hasher_0 = DefaultHasher::new();
    borrowed.hash(&mut hasher_0);

    let owned = path_buf_id!(BTest; "a");
    let mut hasher_1 = DefaultHasher::new();
    owned.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn deref_test() {
    let id = path_buf_id!(BTest; "abc");

    let actual: &PathId<BTest> = id.deref();
    let expected = path_id!(BTest; "abc");
    assert_eq!(actual, expected);

    let actual: &Path = id.as_path();
    let expected = Path::new("abc");
    assert_eq!(actual, expected);
}

#[test]
fn borrow_test() {
    let id = path_buf_id!(BTest; "a");

    let actual: &PathId<BTest> = id.borrow();
    let expected = path_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrow_hash_map_lookup_test() {
    let mut map: HashMap<PathBufId<BTest>, i32> = HashMap::new();
    map.insert(path_buf_id!(BTest; "a"), 1);

    let actual: Option<&i32> = map.get(path_id!(BTest; "a"));
    let expected = Some(&1);
    assert_eq!(actual, expected);
}

#[test]
fn to_owned_test() {
    let borrowed = path_id!(BTest; "a");

    let actual: PathBufId<BTest> = borrowed.to_owned();
    let expected = path_buf_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_as_ref_test() {
    let id = path_id!(BTest; "a");

    let actual: &Path = AsRef::<Path>::as_ref(id);
    let expected = Path::new("a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_as_ref_test() {
    let id = path_buf_id!(BTest; "a");

    let actual: &Path = AsRef::<Path>::as_ref(&id);
    let expected = Path::new("a");
    assert_eq!(actual, expected);

    let actual: &PathId<BTest> = AsRef::<PathId<BTest>>::as_ref(&id);
    let expected = path_id!(BTest; "a");
    assert_eq!(actual, expected);
}
