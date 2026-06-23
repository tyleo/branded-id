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
