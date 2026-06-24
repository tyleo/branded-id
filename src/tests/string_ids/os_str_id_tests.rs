use crate::{OsStrId, OsStringId, os_str_id, os_string_id, tests::util::BTest};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::{HashMap, hash_map::DefaultHasher},
    ffi::{OsStr, OsString},
    hash::{Hash, Hasher},
    ops::Deref,
};

#[test]
fn from_os_str_test() {
    let actual: &OsStrId<BTest> = OsStrId::<BTest>::from_os_str(OsStr::new("a"));
    let expected = os_str_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn as_os_str_test() {
    let id = os_str_id!(BTest; "a");

    let actual: &OsStr = id.as_os_str();
    let expected = OsStr::new("a");
    assert_eq!(actual, expected);
}

#[test]
fn from_os_string_test() {
    let actual: OsStringId<BTest> = OsStringId::<BTest>::from_os_string(OsString::from("a"));
    let expected = os_string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn into_os_string_test() {
    let id = os_string_id!(BTest; "a");

    let actual: OsString = id.into_os_string();
    let expected = OsString::from("a");
    assert_eq!(actual, expected);
}

#[test]
fn from_os_str_ref_test() {
    let actual: &OsStrId<BTest> = From::from(OsStr::new("a"));
    let expected = os_str_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_os_string_value_test() {
    let actual: OsStringId<BTest> = From::from(OsString::from("a"));
    let expected = os_string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_str_slice_test() {
    let actual: OsStringId<BTest> = From::from("a");
    let expected = os_string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_string_test() {
    let actual: OsStringId<BTest> = From::from(String::from("a"));
    let expected = os_string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_os_str_owned_test() {
    let actual: OsStringId<BTest> = From::from(OsStr::new("a"));
    let expected = os_string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_debug_fmt_test() {
    let id = os_str_id!(BTest; "a");

    let actual: String = format!("{:?}", id);
    let expected = "BTest(\"a\")";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = "branded_id::tests::util::b_test::BTest(\"a\")";
    assert_eq!(actual, expected);
}

#[test]
fn owned_debug_fmt_test() {
    let id = os_string_id!(BTest; "a");

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
    let id = os_string_id!(BTest; "a");

    let actual: OsStringId<BTest> = id.clone();
    let expected = os_string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_eq_test() {
    let id_0 = os_str_id!(BTest; "a");
    let id_1 = os_str_id!(BTest; "b");

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn owned_eq_test() {
    let id_0 = os_string_id!(BTest; "a");
    let id_1 = os_string_id!(BTest; "b");

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_cmp_test() {
    let id_0 = os_str_id!(BTest; "a");
    let id_1 = os_str_id!(BTest; "b");

    let actual: Ordering = id_0.cmp(id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_0.partial_cmp(id_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);
}

#[test]
fn owned_cmp_test() {
    let id_0 = os_string_id!(BTest; "a");
    let id_1 = os_string_id!(BTest; "b");

    let actual: Ordering = id_0.cmp(&id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_hash_matches_owned_test() {
    let borrowed = os_str_id!(BTest; "a");
    let mut hasher_0 = DefaultHasher::new();
    borrowed.hash(&mut hasher_0);

    let owned = os_string_id!(BTest; "a");
    let mut hasher_1 = DefaultHasher::new();
    owned.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn deref_test() {
    let id = os_string_id!(BTest; "abc");

    let actual: &OsStrId<BTest> = id.deref();
    let expected = os_str_id!(BTest; "abc");
    assert_eq!(actual, expected);

    let actual: &OsStr = id.as_os_str();
    let expected = OsStr::new("abc");
    assert_eq!(actual, expected);
}

#[test]
fn borrow_test() {
    let id = os_string_id!(BTest; "a");

    let actual: &OsStrId<BTest> = id.borrow();
    let expected = os_str_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrow_hash_map_lookup_test() {
    let mut map: HashMap<OsStringId<BTest>, i32> = HashMap::new();
    map.insert(os_string_id!(BTest; "a"), 1);

    let actual: Option<&i32> = map.get(os_str_id!(BTest; "a"));
    let expected = Some(&1);
    assert_eq!(actual, expected);
}

#[test]
fn to_owned_test() {
    let borrowed = os_str_id!(BTest; "a");

    let actual: OsStringId<BTest> = borrowed.to_owned();
    let expected = os_string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_as_ref_test() {
    let id = os_str_id!(BTest; "a");

    let actual: &OsStr = AsRef::<OsStr>::as_ref(id);
    let expected = OsStr::new("a");
    assert_eq!(actual, expected);
}

#[test]
fn owned_as_ref_test() {
    let id = os_string_id!(BTest; "a");

    let actual: &OsStr = AsRef::<OsStr>::as_ref(&id);
    let expected = OsStr::new("a");
    assert_eq!(actual, expected);

    let actual: &OsStrId<BTest> = AsRef::<OsStrId<BTest>>::as_ref(&id);
    let expected = os_str_id!(BTest; "a");
    assert_eq!(actual, expected);
}
