use crate::{StrId, StringId, str_id, string_id, tests::util::BTest};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    ops::Deref,
    str::FromStr,
};

#[test]
fn from_str_test() {
    let actual: &StrId<BTest> = StrId::<BTest>::from_str("a");
    let expected = str_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn as_str_test() {
    let id = str_id!(BTest; "a");

    let actual: &str = id.as_str();
    let expected = "a";
    assert_eq!(actual, expected);
}

#[test]
fn from_string_test() {
    let actual: StringId<BTest> = StringId::<BTest>::from_string(String::from("a"));
    let expected = string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn into_string_test() {
    let id = string_id!(BTest; "a");

    let actual: String = id.into_string();
    let expected = String::from("a");
    assert_eq!(actual, expected);
}

#[test]
fn from_str_ref_test() {
    let actual: &StrId<BTest> = From::from("a");
    let expected = str_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_string_value_test() {
    let actual: StringId<BTest> = From::from(String::from("a"));
    let expected = string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_str_slice_test() {
    let actual: StringId<BTest> = From::from("a");
    let expected = string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_debug_fmt_test() {
    let id = str_id!(BTest; "a");

    let actual: String = format!("{:?}", id);
    let expected = "BTest(\"a\")";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = "branded_id::tests::util::b_test::BTest(\"a\")";
    assert_eq!(actual, expected);
}

#[test]
fn owned_debug_fmt_test() {
    let id = string_id!(BTest; "a");

    let actual: String = format!("{:?}", id);
    let expected = "BTest(\"a\")";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = "branded_id::tests::util::b_test::BTest(\"a\")";
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_display_fmt_test() {
    let id = str_id!(BTest; "a");

    let actual: String = format!("{}", id);
    let expected = "BTest(a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#}", id);
    let expected = "branded_id::tests::util::b_test::BTest(a)";
    assert_eq!(actual, expected);
}

#[test]
fn owned_display_fmt_test() {
    let id = string_id!(BTest; "a");

    let actual: String = format!("{}", id);
    let expected = "BTest(a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#}", id);
    let expected = "branded_id::tests::util::b_test::BTest(a)";
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let id = string_id!(BTest; "a");

    let actual: StringId<BTest> = id.clone();
    let expected = string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn from_str_trait_test() {
    let actual: StringId<BTest> = <StringId<BTest> as FromStr>::from_str("a").unwrap();
    let expected = string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_eq_test() {
    let id_0 = str_id!(BTest; "a");
    let id_1 = str_id!(BTest; "b");

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn owned_eq_test() {
    let id_0 = string_id!(BTest; "a");
    let id_1 = string_id!(BTest; "b");

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_cmp_test() {
    let id_0 = str_id!(BTest; "a");
    let id_1 = str_id!(BTest; "b");

    let actual: Ordering = id_0.cmp(id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_0.partial_cmp(id_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);
}

#[test]
fn owned_cmp_test() {
    let id_0 = string_id!(BTest; "a");
    let id_1 = string_id!(BTest; "b");

    let actual: Ordering = id_0.cmp(&id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_0.partial_cmp(&id_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_hash_matches_owned_test() {
    let borrowed = str_id!(BTest; "a");
    let mut hasher_0 = DefaultHasher::new();
    borrowed.hash(&mut hasher_0);

    let owned = string_id!(BTest; "a");
    let mut hasher_1 = DefaultHasher::new();
    owned.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn deref_test() {
    let id = string_id!(BTest; "abc");

    let actual: &StrId<BTest> = id.deref();
    let expected = str_id!(BTest; "abc");
    assert_eq!(actual, expected);

    // Deref coercion reaches the borrowed id's inherent methods.
    let actual: &str = id.as_str();
    let expected = "abc";
    assert_eq!(actual, expected);
}

#[test]
fn borrow_test() {
    let id = string_id!(BTest; "a");

    let actual: &StrId<BTest> = id.borrow();
    let expected = str_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrow_hash_map_lookup_test() {
    let mut map: HashMap<StringId<BTest>, i32> = HashMap::new();
    map.insert(string_id!(BTest; "a"), 1);

    // Look up with a borrowed branded id without allocating an owned key.
    let actual: Option<&i32> = map.get(str_id!(BTest; "a"));
    let expected = Some(&1);
    assert_eq!(actual, expected);
}

#[test]
fn to_owned_test() {
    let borrowed = str_id!(BTest; "a");

    let actual: StringId<BTest> = borrowed.to_owned();
    let expected = string_id!(BTest; "a");
    assert_eq!(actual, expected);
}

#[test]
fn borrowed_as_ref_test() {
    let id = str_id!(BTest; "a");

    let actual: &str = AsRef::<str>::as_ref(id);
    let expected = "a";
    assert_eq!(actual, expected);
}

#[test]
fn owned_as_ref_test() {
    let id = string_id!(BTest; "a");

    let actual: &str = AsRef::<str>::as_ref(&id);
    let expected = "a";
    assert_eq!(actual, expected);

    let actual: &StrId<BTest> = AsRef::<StrId<BTest>>::as_ref(&id);
    let expected = str_id!(BTest; "a");
    assert_eq!(actual, expected);
}
