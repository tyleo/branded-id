use crate::{soa::IdStruct, tests::util::BTest, u32_id, usize_id};

// The brand-only form defaults the integer width to `u32`, handing out
// `U32Id`, so it is interchangeable with the `U32IdStruct` alias.
#[test]
fn default_width_is_u32_test() {
    let mut ids = IdStruct::<BTest>::new();

    let id_0 = ids.retain();
    assert_eq!(id_0, u32_id!(BTest; 0));
    assert_eq!(ids.peek_next_fresh(), u32_id!(BTest; 1));
}

// A non-default width keys the same pool by `usize`, handing out `UsizeId`,
// and recycles ids the same way.
#[test]
fn usize_width_retain_release_test() {
    let mut ids = IdStruct::<BTest, usize>::new();

    let id_0 = ids.retain();
    assert_eq!(id_0, usize_id!(BTest; 0));

    let id_1 = ids.retain();
    assert_eq!(id_1, usize_id!(BTest; 1));

    assert!(ids.is_retained(id_0));
    assert!(ids.is_retained(id_1));

    // Releasing recycles the id and swap-removes it from the live region.
    ids.release(id_0);
    assert!(!ids.is_retained(id_0));

    let live: Vec<_> = ids.iter().collect();
    assert_eq!(live, vec![id_1]);

    // The released id is handed back out before a fresh one is allocated.
    assert_eq!(ids.retain(), id_0);
    assert_eq!(ids.peek_next_fresh(), usize_id!(BTest; 2));
}

// Cloning a pool copies its retained and recycled state, and the clone is
// independent of the original.
#[test]
fn clone_test() {
    let mut ids = IdStruct::<BTest>::new();
    let id_0 = ids.retain();
    ids.retain();
    ids.release(id_0);

    let clone = ids.clone();

    assert_eq!(ids, clone);
    let actual: Vec<_> = clone.iter().collect();
    let expected: Vec<_> = ids.iter().collect();
    assert_eq!(actual, expected);

    // Retaining on the clone recycles id_0 and leaves the original untouched.
    let mut clone = clone;
    assert_eq!(clone.retain(), id_0);
    assert_ne!(ids, clone);
    assert_eq!(ids.len(), 1);
}

// Equality is structural over the internal layout: same retain/release history
// is equal, and any divergence is not.
#[test]
fn eq_test() {
    let mut a = IdStruct::<BTest>::new();
    a.retain();
    a.retain();

    let mut b = IdStruct::<BTest>::new();
    b.retain();
    b.retain();

    assert_eq!(a, b);

    a.release(u32_id!(BTest; 0));
    assert_ne!(a, b);
}

// Hash agrees with Eq, so a clone is found in a set keyed by the pool itself.
#[test]
fn hash_test() {
    use std::collections::HashSet;

    let mut ids = IdStruct::<BTest>::new();
    ids.retain();
    ids.retain();

    let mut set = HashSet::new();
    set.insert(ids.clone());
    assert!(set.contains(&ids));
}

// Debug shows the retained (`live`) and recycled-next (`free`) ids; the released
// id_0 was swap-removed to the front of `free`.
#[test]
fn debug_test() {
    let mut ids = IdStruct::<BTest>::new();
    let id_0 = ids.retain();
    ids.retain();
    ids.retain();
    ids.release(id_0);

    let actual = format!("{:?}", ids);
    let expected = "IdStruct { live: [BTest(2), BTest(1)], free: [BTest(0)] }";
    assert_eq!(actual, expected);
}

// A cloned iterator resumes from the same cursor position, independently.
#[test]
fn iter_clone_test() {
    let mut ids = IdStruct::<BTest>::new();
    let id_0 = ids.retain();
    let id_1 = ids.retain();

    let mut iter = ids.iter();
    assert_eq!(iter.next(), Some(id_0));
    let clone = iter.clone();

    assert_eq!(iter.collect::<Vec<_>>(), vec![id_1]);
    assert_eq!(clone.collect::<Vec<_>>(), vec![id_1]);
}

// The iterator's Debug lists the ids it has yet to yield.
#[test]
fn iter_debug_test() {
    let mut ids = IdStruct::<BTest>::new();
    ids.retain();
    ids.retain();

    let actual = format!("{:?}", ids.iter());
    let expected = "IdStructIter([BTest(0), BTest(1)])";
    assert_eq!(actual, expected);
}

// The borrowed raw-parts view is Copy and Debug, rendering all three lists.
#[test]
fn raw_parts_traits_test() {
    let mut ids = IdStruct::<BTest>::new();
    let id_0 = ids.retain();
    ids.retain();
    ids.release(id_0);

    let parts = ids.as_raw_parts();
    let copy = parts;
    assert_eq!(parts, copy);

    let actual = format!("{:?}", parts);
    let expected = "IdStructRawParts { live: [BTest(1)], sparse: BTest[1, 0], free: [BTest(0)] }";
    assert_eq!(actual, expected);
}
