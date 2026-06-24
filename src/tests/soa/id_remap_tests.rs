use crate::{soa::IdStruct, tests::util::BTest, u32_id};

// Builds a remap by retaining 0, 1, 2 then releasing id_1, leaving the live
// set [id_0, id_2] which gc relabels to 0 and 1.
fn remap() -> crate::soa::IdRemap<BTest, u32> {
    let mut ids = IdStruct::<BTest>::new();
    ids.retain();
    let id_1 = ids.retain();
    ids.retain();
    ids.release(id_1);
    ids.gc()
}

// new_id translates live ids, reports released ids as None, and reports ids
// past the covered space as None too.
#[test]
fn new_id_test() {
    let remap = remap();

    assert_eq!(remap.new_id(u32_id!(BTest; 0)), Some(u32_id!(BTest; 0)));
    assert_eq!(remap.new_id(u32_id!(BTest; 2)), Some(u32_id!(BTest; 1)));
    assert_eq!(remap.new_id(u32_id!(BTest; 1)), None);
    assert_eq!(remap.new_id(u32_id!(BTest; 99)), None);
}

// new_len counts the live ids, old_len covers the pre-gc id space, and a remap
// with live ids is not empty.
#[test]
fn lengths_test() {
    let remap = remap();

    assert_eq!(remap.new_len(), 2);
    assert_eq!(remap.old_len(), 3);
    assert!(!remap.is_empty());
}

// A clone is an independent, equal copy.
#[test]
fn clone_test() {
    let remap = remap();
    let clone = remap.clone();

    assert_eq!(remap, clone);
    assert_eq!(clone.new_id(u32_id!(BTest; 2)), Some(u32_id!(BTest; 1)));
}

// Default is the empty remap, covering nothing.
#[test]
fn default_test() {
    let remap = crate::soa::IdRemap::<BTest, u32>::default();

    assert!(remap.is_empty());
    assert_eq!(remap.new_len(), 0);
    assert_eq!(remap.old_len(), 0);
    assert_eq!(remap.new_id(u32_id!(BTest; 0)), None);
}

// Debug renders the per-old-id relabeling and the live count.
#[test]
fn debug_test() {
    let actual = format!("{:?}", remap());
    let expected = "IdRemap { new_ids: BTest[Some(BTest(0)), None, Some(BTest(1))], new_len: 2 }";
    assert_eq!(actual, expected);
}

// Hash agrees with Eq, so a clone is found in a set keyed by the remap itself.
#[test]
fn hash_test() {
    use std::collections::HashSet;

    let remap = remap();

    let mut set = HashSet::new();
    set.insert(remap.clone());
    assert!(set.contains(&remap));
}

// Equality is structural: remaps from the same history are equal, a different
// release history is not.
#[test]
fn eq_test() {
    assert_eq!(remap(), remap());

    let mut ids = IdStruct::<BTest>::new();
    ids.retain();
    ids.retain();
    ids.retain();
    let different = ids.gc();

    assert_ne!(remap(), different);
}
