use crate::{U128Id, UuidId, tests::util::BTest, u128_id, uuid_id as id};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::FromStr,
};
use uuid::Uuid;

const UUID_STR: &str = "01234567-89ab-cdef-0123-456789abcdef";

fn uuid_a() -> Uuid {
    Uuid::from_u128(0x0123456789abcdef0123456789abcdef)
}

fn uuid_b() -> Uuid {
    Uuid::from_u128(0xfedcba9876543210fedcba9876543210)
}

#[test]
fn from_uuid_test() {
    let actual: UuidId<BTest> = UuidId::<BTest>::from_uuid(uuid_a());
    let expected = id!(BTest; uuid_a());
    assert_eq!(actual, expected);
}

#[test]
fn to_uuid_test() {
    let id = id!(BTest; uuid_a());

    let actual: Uuid = id.to_uuid();
    let expected = uuid_a();
    assert_eq!(actual, expected);
}

#[test]
fn to_u128_id_test() {
    let id = id!(BTest; uuid_a());

    let actual: U128Id<BTest> = id.to_u128_id();
    let expected = u128_id!(BTest; 0x0123456789abcdef0123456789abcdef);
    assert_eq!(actual, expected);
}

#[test]
fn to_uuid_id_test() {
    let id = u128_id!(BTest; 0x0123456789abcdef0123456789abcdef);

    let actual: UuidId<BTest> = id.to_uuid_id();
    let expected = id!(BTest; uuid_a());
    assert_eq!(actual, expected);
}

#[test]
fn from_test() {
    let actual: UuidId<BTest> = From::from(uuid_a());
    let expected = id!(BTest; uuid_a());
    assert_eq!(actual, expected);
}

#[test]
fn into_uuid_test() {
    let id = id!(BTest; uuid_a());

    let actual: Uuid = Uuid::from(id);
    let expected = uuid_a();
    assert_eq!(actual, expected);
}

#[test]
fn from_str_test() {
    let actual: UuidId<BTest> = <UuidId<BTest> as FromStr>::from_str(UUID_STR).unwrap();
    let expected = id!(BTest; uuid_a());
    assert_eq!(actual, expected);
}

#[test]
fn as_ref_test() {
    let id = id!(BTest; uuid_a());

    let actual: &Uuid = AsRef::<Uuid>::as_ref(&id);
    let expected = &uuid_a();
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let id = id!(BTest; uuid_a());

    let actual: UuidId<BTest> = id.clone();
    let expected = id!(BTest; uuid_a());
    assert_eq!(actual, expected);
}

#[test]
fn debug_fmt_test() {
    let id = id!(BTest; uuid_a());

    let actual: String = format!("{:?}", id);
    let expected = format!("BTest({})", UUID_STR);
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = format!("branded_id::tests::util::b_test::BTest({})", UUID_STR);
    assert_eq!(actual, expected);
}

#[test]
fn display_fmt_test() {
    let id = id!(BTest; uuid_a());

    let actual: String = format!("{}", id);
    let expected = format!("BTest({})", UUID_STR);
    assert_eq!(actual, expected);

    let actual: String = format!("{:#}", id);
    let expected = format!("branded_id::tests::util::b_test::BTest({})", UUID_STR);
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_0 = id!(BTest; uuid_a());
    let id_1 = id!(BTest; uuid_b());

    let actual: bool = id_0 == id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_test() {
    let id_0 = id!(BTest; uuid_a());
    let id_1 = id!(BTest; uuid_b());

    let actual: bool = id_0 != id_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_0 != id_1;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn cmp_test() {
    let id_0 = id!(BTest; uuid_a());
    let id_1 = id!(BTest; uuid_b());

    let actual: Ordering = id_0.cmp(&id_0);
    let expected = Ordering::Equal;
    assert_eq!(actual, expected);

    let actual: Ordering = id_0.cmp(&id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Ordering = id_1.cmp(&id_0);
    let expected = Ordering::Greater;
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let id_0 = id!(BTest; uuid_a());
    let id_1 = id!(BTest; uuid_b());

    let actual: Option<Ordering> = id_0.partial_cmp(&id_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);
}

#[test]
fn hash_test() {
    let id = id!(BTest; uuid_a());
    let mut hasher_0 = DefaultHasher::new();
    id.hash(&mut hasher_0);

    let uuid = uuid_a();
    let mut hasher_1 = DefaultHasher::new();
    uuid.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}
