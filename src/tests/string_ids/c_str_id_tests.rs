use crate::{CStrId, CStringId, c_str_id, c_string_id, tests::util::BTest};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::{HashMap, hash_map::DefaultHasher},
    ffi::{CStr, CString},
    hash::{Hash, Hasher},
    ops::Deref,
};

#[test]
fn from_c_str_test() {
    let actual: &CStrId<BTest> = CStrId::<BTest>::from_c_str(c"a");
    let expected = c_str_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn as_c_str_test() {
    let id = c_str_id!(BTest; c"a");

    let actual: &CStr = id.as_c_str();
    let expected = c"a";
    assert_eq!(actual, expected);
}

#[test]
fn from_c_string_test() {
    let actual: CStringId<BTest> = CStringId::<BTest>::from_c_string(CString::from(c"a"));
    let expected = c_string_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn into_c_string_test() {
    let id = c_string_id!(BTest; c"a");

    let actual: CString = id.into_c_string();
    let expected = CString::from(c"a");
    assert_eq!(actual, expected);
}

#[test]
fn from_c_str_ref_test() {
    let actual: &CStrId<BTest> = From::from(c"a");
    let expected = c_str_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn from_c_string_value_test() {
    let actual: CStringId<BTest> = From::from(CString::from(c"a"));
    let expected = c_string_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn from_c_str_owned_test() {
    let actual: CStringId<BTest> = From::from(c"a");
    let expected = c_string_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_debug_fmt_test() {
    let id = c_str_id!(BTest; c"a");

    let actual: String = format!("{:?}", id);
    let expected = "BTest(\"a\")";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = "branded_id::tests::util::b_test::BTest(\"a\")";
    assert_eq!(actual, expected);
}

#[test]
fn owned_debug_fmt_test() {
    let id = c_string_id!(BTest; c"a");

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
    let id = c_string_id!(BTest; c"a");

    let actual: CStringId<BTest> = id.clone();
    let expected = c_string_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_eq_test() {
    let id_0 = c_str_id!(BTest; c"a");
    let id_1 = c_str_id!(BTest; c"b");

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn owned_eq_test() {
    let id_0 = c_string_id!(BTest; c"a");
    let id_1 = c_string_id!(BTest; c"b");

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_cmp_test() {
    let id_0 = c_str_id!(BTest; c"a");
    let id_1 = c_str_id!(BTest; c"b");

    let actual: Ordering = id_0.cmp(id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_0.partial_cmp(id_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);
}

#[test]
fn owned_cmp_test() {
    let id_0 = c_string_id!(BTest; c"a");
    let id_1 = c_string_id!(BTest; c"b");

    let actual: Ordering = id_0.cmp(&id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_hash_matches_owned_test() {
    let borrowed = c_str_id!(BTest; c"a");
    let mut hasher_0 = DefaultHasher::new();
    borrowed.hash(&mut hasher_0);

    let owned = c_string_id!(BTest; c"a");
    let mut hasher_1 = DefaultHasher::new();
    owned.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn deref_test() {
    let id = c_string_id!(BTest; c"abc");

    let actual: &CStrId<BTest> = id.deref();
    let expected = c_str_id!(BTest; c"abc");
    assert_eq!(actual, expected);

    let actual: &CStr = id.as_c_str();
    let expected = c"abc";
    assert_eq!(actual, expected);
}

#[test]
fn borrow_test() {
    let id = c_string_id!(BTest; c"a");

    let actual: &CStrId<BTest> = id.borrow();
    let expected = c_str_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn borrow_hash_map_lookup_test() {
    let mut map: HashMap<CStringId<BTest>, i32> = HashMap::new();
    map.insert(c_string_id!(BTest; c"a"), 1);

    let actual: Option<&i32> = map.get(c_str_id!(BTest; c"a"));
    let expected = Some(&1);
    assert_eq!(actual, expected);
}

#[test]
fn to_owned_test() {
    let borrowed = c_str_id!(BTest; c"a");

    let actual: CStringId<BTest> = borrowed.to_owned();
    let expected = c_string_id!(BTest; c"a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_as_ref_test() {
    let id = c_str_id!(BTest; c"a");

    let actual: &CStr = AsRef::<CStr>::as_ref(id);
    let expected = c"a";
    assert_eq!(actual, expected);
}

#[test]
fn owned_as_ref_test() {
    let id = c_string_id!(BTest; c"a");

    let actual: &CStr = AsRef::<CStr>::as_ref(&id);
    let expected = c"a";
    assert_eq!(actual, expected);

    let actual: &CStrId<BTest> = AsRef::<CStrId<BTest>>::as_ref(&id);
    let expected = c_str_id!(BTest; c"a");
    assert_eq!(actual, expected);
}
