use crate::{
    soa::{IdField, UsizeIdStruct},
    tests::util::MTest,
    usize_id,
};

#[test]
fn clear_test() {
    let mut field = IdField::<MTest, u32>::new();

    field.retain(usize_id!(MTest; 0), 1);
    field.retain(usize_id!(MTest; 1), 2);
    assert_eq!(field.reserved_count(), 2);

    field.clear();
    assert_eq!(field.reserved_count(), 0);

    // Usable again after clear; allocation starts fresh.
    let actual = *field.retain(usize_id!(MTest; 0), 42);
    assert_eq!(actual, 42);
    assert_eq!(*unsafe { field.get(usize_id!(MTest; 0)) }, 42);
}

#[test]
fn ensure_count_test() {
    let mut field = IdField::<MTest, u32>::new();

    field.ensure_count(4);
    assert_eq!(field.reserved_count(), 4);

    // Never shrinks.
    field.ensure_count(2);
    assert_eq!(field.reserved_count(), 4);
}

#[test]
fn is_reserved_test() {
    let mut field = IdField::<MTest, u32>::new();

    assert!(!field.is_reserved(usize_id!(MTest; 0)));

    field.retain(usize_id!(MTest; 0), 1);

    assert!(field.is_reserved(usize_id!(MTest; 0)));
    assert!(!field.is_reserved(usize_id!(MTest; 1)));
}

#[test]
fn new_test() {
    IdField::<MTest, u32>::new();
}

#[test]
fn new_with_capacity_test() {
    // Capacity reserves storage but does not populate it.
    let field = IdField::<MTest, u32>::new_with_capacity(8);
    assert_eq!(field.reserved_count(), 0);
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

    let actual = *unsafe { health.get(id_0) };
    let expected = 1;

    assert_eq!(actual, expected);
}

#[test]
fn release_zeroed_test() {
    let mut field = IdField::<MTest, u32>::new();
    let id_0 = usize_id!(MTest; 0);

    field.retain(id_0, 7);
    unsafe { field.release_zeroed(id_0) };

    // The slot stays reserved, but its bytes were clobbered with zeros.
    assert_eq!(field.reserved_count(), 1);
    assert_eq!(*unsafe { field.get(id_0) }, 0);
}

#[test]
fn reserved_count_test() {
    let mut field = IdField::<MTest, u32>::new();
    assert_eq!(field.reserved_count(), 0);

    field.retain(usize_id!(MTest; 0), 10);
    assert_eq!(field.reserved_count(), 1);

    // Retaining a higher id reserves every slot up to it.
    field.retain(usize_id!(MTest; 5), 20);
    assert_eq!(field.reserved_count(), 6);
}

#[test]
fn retain_test() {
    let mut obj = UsizeIdStruct::<MTest>::new();
    let mut health = IdField::new();

    let id_0 = obj.retain();
    health.retain(id_0, 1);

    let actual = *unsafe { health.get(id_0) };
    let expected = 1;

    assert_eq!(actual, expected);
}
