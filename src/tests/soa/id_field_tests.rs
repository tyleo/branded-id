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

// A field value that records each drop in a shared counter, so the gc test can
// prove it neither leaks nor double-drops a value without reading the
// moved-out slots (which would itself be undefined behavior under a buggy gc).
struct DropCount {
    value: u32,
    drops: std::rc::Rc<std::cell::Cell<usize>>,
}

impl DropCount {
    fn new(value: u32, drops: &std::rc::Rc<std::cell::Cell<usize>>) -> Self {
        Self {
            value,
            drops: std::rc::Rc::clone(drops),
        }
    }
}

impl Drop for DropCount {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

// gc moves each live value to its relabeled id following the pool's iteration
// order (not ascending old-id order), shrinks the field to the live count, and
// neither leaks nor double-drops along the way.
#[test]
fn gc_test() {
    use std::cell::Cell;
    use std::rc::Rc;

    let drops = Rc::new(Cell::new(0));

    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, DropCount>::new();

    let id_0 = ids.retain();
    field.retain(id_0, DropCount::new(10, &drops));
    let id_1 = ids.retain();
    field.retain(id_1, DropCount::new(20, &drops));
    let id_2 = ids.retain();
    field.retain(id_2, DropCount::new(30, &drops));

    // Release the first id from both. Swap-remove leaves the live order
    // [id_2, id_1], so gc genuinely reorders the survivors rather than leaving
    // them in ascending old-id order.
    unsafe { field.release(id_0) };
    ids.release(id_0);
    assert_eq!(drops.get(), 1);

    let remap = ids.gc();
    // SAFETY: `field` is in sync with `ids`'s pre-gc state.
    unsafe { field.gc(&remap) };

    // The move dropped nothing: only the earlier release is counted.
    assert_eq!(drops.get(), 1);

    // Storage shrank to the two live ids, each placed at the pool's iteration
    // index: id_2's value leads (new id 0), id_1's follows (new id 1).
    assert_eq!(field.reserved_count(), 2);
    let new_1 = remap.new_id(id_1).unwrap();
    let new_2 = remap.new_id(id_2).unwrap();
    assert_eq!(new_2, u32_id!(BTest; 0));
    assert_eq!(new_1, u32_id!(BTest; 1));
    assert_eq!(unsafe { field.get(new_2) }.value, 30);
    assert_eq!(unsafe { field.get(new_1) }.value, 20);

    // Iteration yields the live values in the relabeled order.
    let actual: Vec<u32> = unsafe { field.iter(&ids) }.map(|v| v.value).collect();
    assert_eq!(actual, vec![30, 20]);

    // Clearing drops exactly the two survivors, bringing the total to three:
    // every value dropped once, so the move neither leaked nor double-dropped.
    unsafe { field.clear(&ids) };
    ids.clear();
    assert_eq!(drops.get(), 3);
}

// gc against an empty remap leaves an empty field.
#[test]
fn gc_empty_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, u32>::new();

    let remap = ids.gc();
    // SAFETY: an empty field is trivially in sync with an empty pool.
    unsafe { field.gc(&remap) };

    assert_eq!(field.reserved_count(), 0);
}

// gc after every id was released drops nothing again and empties the field,
// even though its backing storage still spans the released slots.
#[test]
fn gc_all_released_test() {
    use std::rc::Rc;

    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, Rc<()>>::new();

    let token = Rc::new(());

    let id_0 = ids.retain();
    field.retain(id_0, Rc::clone(&token));
    let id_1 = ids.retain();
    field.retain(id_1, Rc::clone(&token));

    // Drop the values, then release the ids from the pool.
    unsafe { field.release_all(&ids) };
    ids.release(id_0);
    ids.release(id_1);
    assert_eq!(Rc::strong_count(&token), 1);

    // Storage still spans the released slots before gc, so the post-gc count
    // demonstrates a real 2 -> 0 shrink.
    assert_eq!(field.reserved_count(), 2);

    let remap = ids.gc();
    // SAFETY: the field released every value, matching the now-empty pool.
    unsafe { field.gc(&remap) };

    // The already-dropped slots are discarded without dropping again.
    assert_eq!(field.reserved_count(), 0);
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
fn filled_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    // A pool with three live ids matching the three filled slots.
    let id_0 = ids.retain();
    let id_1 = ids.retain();
    let id_2 = ids.retain();

    let mut field = IdField::<BTest, u32>::filled(3, 7);

    // Every filled slot is reserved and reads back the fill value.
    assert_eq!(field.reserved_count(), 3);
    assert_eq!(*unsafe { field.get(id_0) }, 7);
    assert_eq!(*unsafe { field.get(id_1) }, 7);
    assert_eq!(*unsafe { field.get(id_2) }, 7);

    // Tidy up so the filled values aren't leaked.
    unsafe { field.clear(&ids) };
}

#[test]
fn filled_empty_test() {
    let field = IdField::<BTest, u32>::filled(0, 7);
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

// Cloning a field of Copy values duplicates the reserved storage bytewise, so
// the clone reads back the same values at the live ids.
#[test]
fn clone_test() {
    let mut ids = U32IdStruct::<BTest>::new();
    let mut field = IdField::<BTest, u32>::new();

    let id_0 = ids.retain();
    field.retain(id_0, 10);
    let id_1 = ids.retain();
    field.retain(id_1, 20);

    let clone = field.clone();

    assert_eq!(clone.reserved_count(), field.reserved_count());
    // SAFETY: `clone` is a faithful copy of `field`, which is in sync with `ids`,
    // so id_0 and id_1 are initialized in the clone too.
    assert_eq!(*unsafe { clone.get(id_0) }, 10);
    assert_eq!(*unsafe { clone.get(id_1) }, 20);
}

// Debug reports only the reserved-slot count: the values aren't shown because
// liveness lives in the paired IdStruct, not the field.
#[test]
fn debug_test() {
    let mut field = IdField::<BTest, u32>::new();
    field.retain(u32_id!(BTest; 0), 7);
    field.retain(u32_id!(BTest; 2), 9);

    let actual = format!("{:?}", field);
    let expected = "IdField { reserved_count: 3, .. }";
    assert_eq!(actual, expected);
}
