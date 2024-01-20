use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    id::{tests::marker::MUnknown, Id32I},
    id32i,
};

#[test]
fn from_i32_test() {
    let actual = Id32I::<MUnknown>::from_i32(0);
    let expected = id32i!(MUnknown; 0);

    assert_eq!(actual, expected);
}

#[test]
fn offset_test() {
    let id = id32i!(MUnknown; 0);

    let actual = id.offset(1);
    let expected = id32i!(MUnknown; 1);
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let actual = id32i!(MUnknown; 0);
    let expected = actual.clone();
    assert_eq!(actual, expected);
}

#[test]
fn copy_test() {
    let actual = id32i!(MUnknown; 0);
    let expected = actual;
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);
    let id_2 = id32i!(MUnknown; 0);

    let actual = id_0 == id_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_0 == id_2;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn from_test() {
    let actual: Id32I<MUnknown> = From::from(1);
    let expected = id32i!(MUnknown; 1);
    assert_eq!(actual, expected);
}

#[test]
fn hash_test() {
    let id = id32i!(MUnknown; 0);
    let mut hasher_0 = DefaultHasher::new();
    id.hash(&mut hasher_0);

    let u32 = 0u32;
    let mut hasher_1 = DefaultHasher::new();
    u32.hash(&mut hasher_1);

    let actual = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn cmp_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);
    let id_2 = id32i!(MUnknown; 0);

    let actual = id_0.cmp(&id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual = id_1.cmp(&id_0);
    let expected = Ordering::Greater;
    assert_eq!(actual, expected);

    let actual = id_0.cmp(&id_2);
    let expected = Ordering::Equal;
    assert_eq!(actual, expected);
}

#[test]
fn max_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);

    let actual = id_0.max(id_1);
    let expected = id32i!(1);
    assert_eq!(actual, expected);
}

#[test]
fn min_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);

    let actual = id_0.min(id_1);
    let expected = id32i!(0);
    assert_eq!(actual, expected);
}

#[test]
fn clamp_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);
    let id_2 = id32i!(MUnknown; 2);

    let actual = id_0.clamp(id_1, id_2);
    let expected = id32i!(1);
    assert_eq!(actual, expected);

    let actual = id_2.clamp(id_0, id_1);
    let expected = id32i!(1);
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 0);

    let actual = id_0.partial_cmp(&id_1);
    let expected = Some(Ordering::Equal);
    assert_eq!(actual, expected);
}

#[test]
fn lt_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);
    let id_2 = id32i!(MUnknown; 0);

    let actual = id_0 < id_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_1 < id_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_0 < id_2;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn le_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);
    let id_2 = id32i!(MUnknown; 0);

    let actual = id_0 <= id_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_1 <= id_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_0 <= id_2;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn gt_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);
    let id_2 = id32i!(MUnknown; 0);

    let actual = id_0 > id_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_1 > id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_0 > id_2;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ge_test() {
    let id_0 = id32i!(MUnknown; 0);
    let id_1 = id32i!(MUnknown; 1);
    let id_2 = id32i!(MUnknown; 0);

    let actual = id_0 >= id_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_1 >= id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_0 >= id_2;
    let expected = true;
    assert_eq!(actual, expected);
}
