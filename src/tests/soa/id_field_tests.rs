use crate::{
    soa::{IdField, U32IdStruct},
    tests::util::BTest,
    u32_id,
};

#[test]
fn clear_test() {
    use std::rc::Rc;

    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, Rc<()>>::new();

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
fn is_reserved_test() {
    let mut field = IdField::<BTest, u32>::new();

    assert!(!field.is_reserved(u32_id!(BTest; 0)));

    field.retain(u32_id!(BTest; 0), 1);

    assert!(field.is_reserved(u32_id!(BTest; 0)));
    assert!(!field.is_reserved(u32_id!(BTest; 1)));
}

#[test]
fn iter_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, u32>::new();

    let id_0 = ids.retain();
    field.retain(id_0, 10);
    let id_1 = ids.retain();
    field.retain(id_1, 20);
    let id_2 = ids.retain();
    field.retain(id_2, 30);

    // `ids` iterates in live order: id_0, id_1, id_2.
    let actual: Vec<u32> = unsafe { field.iter(&ids) }.copied().collect();
    assert_eq!(actual, vec![10, 20, 30]);
}

#[test]
fn iter_mut_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, u32>::new();

    let id_0 = ids.retain();
    field.retain(id_0, 1);
    let id_1 = ids.retain();
    field.retain(id_1, 2);

    for value in unsafe { field.iter_mut(&ids) } {
        *value *= 10;
    }

    assert_eq!(*unsafe { field.get(id_0) }, 10);
    assert_eq!(*unsafe { field.get(id_1) }, 20);
}

#[test]
#[should_panic(expected = "id is out of range for this field")]
fn iter_mut_out_of_range_panics_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    ids.retain();

    let mut field = IdField::<BTest, u32>::new();

    // SAFETY: deliberately misusing the field/ids pairing to exercise the
    // out-of-range assert in IdFieldIterMut rather than reading uninitialized
    // storage.
    let mut iter = unsafe { field.iter_mut(&ids) };
    let _ = iter.next();
}

#[test]
fn new_test() {
    IdField::<BTest, u32>::new();
}

#[test]
fn with_capacity_test() {
    // Capacity reserves storage but does not populate it.
    let field = IdField::<BTest, u32>::with_capacity(8);
    assert_eq!(field.reserved_count(), 0);
}

#[test]
fn release_test() {
    let mut obj = U32IdStruct::<BTest>::new();
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

    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, Rc<()>>::new();

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
fn release_all_zeroed_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, u32>::new();

    let id_0 = ids.retain();
    field.retain(id_0, 7);
    let id_1 = ids.retain();
    field.retain(id_1, 9);

    // SAFETY: `field` and `ids` are in sync.
    unsafe { field.release_all_zeroed(&ids) };

    // Slots kept, but their bytes were clobbered with zeros.
    assert_eq!(field.reserved_count(), 2);
    assert_eq!(*unsafe { field.get(id_0) }, 0);
    assert_eq!(*unsafe { field.get(id_1) }, 0);
}

#[test]
fn release_zeroed_test() {
    let mut field = IdField::<BTest, u32>::new();
    let id_0 = u32_id!(BTest; 0);

    field.retain(id_0, 7);
    unsafe { field.release_zeroed(id_0) };

    // The slot stays reserved, but its bytes were clobbered with zeros.
    assert_eq!(field.reserved_count(), 1);
    assert_eq!(*unsafe { field.get(id_0) }, 0);
}

#[test]
fn reserve_test() {
    let mut field = IdField::<BTest, u32>::new();

    field.reserve(4);
    assert_eq!(field.reserved_count(), 4);

    // Never shrinks.
    field.reserve(2);
    assert_eq!(field.reserved_count(), 4);
}

#[test]
fn reserved_count_test() {
    let mut field = IdField::<BTest, u32>::new();
    assert_eq!(field.reserved_count(), 0);

    field.retain(u32_id!(BTest; 0), 10);
    assert_eq!(field.reserved_count(), 1);

    // Retaining a higher id reserves every slot up to it.
    field.retain(u32_id!(BTest; 5), 20);
    assert_eq!(field.reserved_count(), 6);
}

#[test]
fn retain_test() {
    let mut obj = U32IdStruct::<BTest>::new();
    let mut health = IdField::new();

    let id_0 = obj.retain();
    health.retain(id_0, 1);

    let actual = *unsafe { health.get(id_0) };
    let expected = 1;

    assert_eq!(actual, expected);
}
