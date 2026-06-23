use crate::{
    soa::{IdField, UsizeIdStruct},
    tests::util::MTest,
    usize_id,
};

#[test]
fn clear_test() {
    use std::rc::Rc;

    let mut ids = UsizeIdStruct::<MTest>::new();
    let mut field = IdField::<MTest, Rc<()>>::new();

    let token = Rc::new(());

    let id_0 = ids.retain();
    field.retain(id_0, Rc::clone(&token));
    let id_1 = ids.retain();
    field.retain(id_1, Rc::clone(&token));

    // token + the two stored clones.
    assert_eq!(Rc::strong_count(&token), 3);
    assert_eq!(field.reserved_count(), 2);

    // SAFETY: `field` and `ids` are in sync.
    unsafe { field.clear(&ids) };
    ids.clear();

    // The stored clones were dropped, not leaked; the field is empty.
    assert_eq!(Rc::strong_count(&token), 1);
    assert_eq!(field.reserved_count(), 0);

    // Usable again after clearing.
    let id_0 = ids.retain();
    let actual = Rc::clone(field.retain(id_0, Rc::clone(&token)));
    assert_eq!(actual, token);
    assert_eq!(Rc::strong_count(&token), 3);

    // Tidy up so the stored clone isn't leaked.
    unsafe { field.clear(&ids) };
    drop(actual);
    assert_eq!(Rc::strong_count(&token), 1);
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
fn release_all_test() {
    use std::rc::Rc;

    let mut ids = UsizeIdStruct::<MTest>::new();
    let mut field = IdField::<MTest, Rc<()>>::new();

    let token = Rc::new(());

    let id_0 = ids.retain();
    field.retain(id_0, Rc::clone(&token));
    let id_1 = ids.retain();
    field.retain(id_1, Rc::clone(&token));

    assert_eq!(Rc::strong_count(&token), 3);

    // SAFETY: `field` and `ids` are in sync.
    unsafe { field.release_all(&ids) };
    ids.clear();

    // Values dropped, but the reserved slots are kept (unlike clear).
    assert_eq!(Rc::strong_count(&token), 1);
    assert_eq!(field.reserved_count(), 2);
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
