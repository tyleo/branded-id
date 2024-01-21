use crate::{i32_id as id, tests::util::MTest, I32Id};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[test]
fn from_i32_test() {
    let actual = I32Id::<MTest>::from_i32(1);
    let expected = id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn offset_test() {
    let id = id!(MTest; 1);

    let actual = id.offset(1);
    let expected = id!(MTest; 2);
    assert_eq!(actual, expected);
}

#[test]
fn to_i32_test() {
    let id = id!(MTest; 1);

    let actual = id.to_i32();
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let actual = id!(MTest; 1);
    let expected = actual.clone();
    assert_eq!(actual, expected);
}

#[test]
fn from_test() {
    let actual: I32Id<MTest> = From::from(1);
    let expected = id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn hash_test() {
    let id = id!(MTest; 1);
    let mut hasher_0 = DefaultHasher::new();
    id.hash(&mut hasher_0);

    let int = 1i32;
    let mut hasher_1 = DefaultHasher::new();
    int.hash(&mut hasher_1);

    let actual = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn hash_slice_test() {
    let ids = [id!(MTest; 1), id!(MTest; 2)];
    let mut hasher_0 = DefaultHasher::new();
    ids.hash(&mut hasher_0);

    let ints = [1i32, 2i32];
    let mut hasher_1 = DefaultHasher::new();
    ints.hash(&mut hasher_1);

    let actual = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn cmp_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 1);

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
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual = id_0.max(id_1);
    let expected = id!(2);
    assert_eq!(actual, expected);
}

#[test]
fn min_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual = id_0.min(id_1);
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn clamp_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 3);

    let actual = id_0.clamp(id_1, id_2);
    let expected = id!(2);
    assert_eq!(actual, expected);

    let actual = id_2.clamp(id_0, id_1);
    let expected = id!(2);
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 1);

    let actual = id_0.eq(&id_1);
    let expected = false;
    assert_eq!(actual, expected);

    let actual = id_0.eq(&id_2);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn ne_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 1);

    let actual = id_0.ne(&id_1);
    let expected = true;
    assert_eq!(actual, expected);

    let actual = id_0.ne(&id_2);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 1);

    let actual = id_0.partial_cmp(&id_1);
    let expected = Some(Ordering::Equal);
    assert_eq!(actual, expected);
}

#[test]
fn lt_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 1);

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
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 1);

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
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 1);

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
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 1);

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
