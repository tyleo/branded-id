use crate::{
    soa::{IdField, UsizeIdStruct},
    tests::util::MTest,
};

#[test]
fn new_test() {
    IdField::<MTest, u32>::new();
}

#[test]
fn retain_test() {
    let mut obj = UsizeIdStruct::<MTest>::new();
    let mut health = IdField::new();

    let id_0 = obj.retain();
    health.retain(id_0, 1);

    let actual = health[id_0];
    let expected = 1;

    assert_eq!(actual, expected);
}

#[test]
fn release_test() {
    let mut obj = UsizeIdStruct::<MTest>::new();
    let mut health = IdField::new();

    let id_0 = obj.retain();
    health.retain(id_0, 1);

    unsafe {
        health.release(id_0);
    }
    obj.release(id_0);

    let id_0 = obj.retain();
    health.retain(id_0, 1);

    let actual = health[id_0];
    let expected = 1;

    assert_eq!(actual, expected);
}
