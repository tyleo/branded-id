use crate::{soa::U32IdStruct, tests::util::MTest, u32_id};

#[test]
fn new_test() {
    let id_struct = U32IdStruct::<MTest>::new();

    let actual = id_struct.count();
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn count_test() {
    let mut id_struct = U32IdStruct::<MTest>::new();

    let actual = id_struct.count();
    let expected = 0;
    assert_eq!(actual, expected);

    let id_0 = id_struct.retain();
    let id_1 = id_struct.retain();

    let actual = id_struct.count();
    let expected = 2;
    assert_eq!(actual, expected);

    id_struct.release(id_0);

    let actual = id_struct.count();
    let expected = 1;
    assert_eq!(actual, expected);

    id_struct.release(id_1);

    let actual = id_struct.count();
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn retain_release_test() {
    let mut id_struct = U32IdStruct::<MTest>::new();

    let id_0 = id_struct.retain();

    let actual = id_0;
    let expected = u32_id!(MTest; 0);
    assert_eq!(actual, expected);

    let id_1 = id_struct.retain();

    let actual = id_1;
    let expected = u32_id!(MTest; 1);
    assert_eq!(actual, expected);

    id_struct.release(id_0);

    let actual = id_struct.retain();
    let expected = u32_id!(MTest; 0);
    assert_eq!(actual, expected);

    id_struct.release(id_0);
    id_struct.release(id_1);

    let actual = id_struct.retain();
    let expected = u32_id!(MTest; 1);
    assert_eq!(actual, expected);

    let actual = id_struct.retain();
    let expected = u32_id!(MTest; 0);
    assert_eq!(actual, expected);
}

#[test]
fn into_iter_test() {
    let mut id_struct = U32IdStruct::<MTest>::new();

    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![];
    assert_eq!(actual, expected);

    let id_0 = id_struct.retain();
    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_0];
    assert_eq!(actual, expected);

    let id_1 = id_struct.retain();
    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_0, id_1];
    assert_eq!(actual, expected);

    let id_2 = id_struct.retain();
    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_0, id_1, id_2];
    assert_eq!(actual, expected);

    id_struct.release(id_0);
    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_2, id_1];
    assert_eq!(actual, expected);

    id_struct.retain();
    id_struct.release(id_1);

    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_2, id_0];
    assert_eq!(actual, expected);

    id_struct.retain();
    id_struct.release(id_2);

    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_1, id_0];
    assert_eq!(actual, expected);

    id_struct.retain();
    id_struct.release(id_0);
    id_struct.release(id_1);

    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_2];
    assert_eq!(actual, expected);

    id_struct.retain();
    id_struct.retain();
    id_struct.release(id_1);
    id_struct.release(id_2);

    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_0];
    assert_eq!(actual, expected);

    id_struct.retain();
    id_struct.retain();
    id_struct.release(id_0);
    id_struct.release(id_2);

    let actual: Vec<_> = id_struct.into_iter().collect();
    let expected = vec![id_1];
    assert_eq!(actual, expected);
}

#[test]
fn is_empty_test() {
    let mut id_struct = U32IdStruct::<MTest>::new();
    assert!(id_struct.is_empty());

    let id_0 = id_struct.retain();
    assert!(!id_struct.is_empty());

    id_struct.release(id_0);
    assert!(id_struct.is_empty());
}

#[test]
fn iter_test() {
    let mut id_struct = U32IdStruct::<MTest>::new();
    let id_0 = id_struct.retain();
    let id_1 = id_struct.retain();

    let actual: Vec<_> = id_struct.iter().collect();
    let expected = vec![id_0, id_1];
    assert_eq!(actual, expected);
}

#[test]
fn is_retained_test() {
    let mut id_struct = U32IdStruct::<MTest>::new();

    // Out of range ids are never retained (and must not panic).
    assert!(!id_struct.is_retained(u32_id!(MTest; 0)));

    let id_0 = id_struct.retain();
    let id_1 = id_struct.retain();

    assert!(id_struct.is_retained(id_0));
    assert!(id_struct.is_retained(id_1));

    id_struct.release(id_0);

    assert!(!id_struct.is_retained(id_0));
    assert!(id_struct.is_retained(id_1));

    // An id past the high-water mark is in-bounds-checked and not retained.
    assert!(!id_struct.is_retained(u32_id!(MTest; 5)));
}

#[test]
fn peek_test() {
    let mut id_struct = U32IdStruct::<MTest>::new();

    // On an empty pool both peeks point at the first fresh id.
    assert_eq!(id_struct.peek_next(), u32_id!(MTest; 0));
    assert_eq!(id_struct.peek_next_fresh(), u32_id!(MTest; 0));

    let id_0 = id_struct.retain();
    assert_eq!(id_0, u32_id!(MTest; 0));

    assert_eq!(id_struct.peek_next(), u32_id!(MTest; 1));
    assert_eq!(id_struct.peek_next_fresh(), u32_id!(MTest; 1));

    id_struct.retain();
    id_struct.release(id_0);

    // peek_next returns the recycled id; peek_next_fresh ignores the free list.
    assert_eq!(id_struct.peek_next(), id_0);
    assert_eq!(id_struct.peek_next_fresh(), u32_id!(MTest; 2));

    // Retaining returns exactly what peek_next predicted.
    assert_eq!(id_struct.retain(), id_0);
}

#[test]
fn clear_test() {
    let mut id_struct = U32IdStruct::<MTest>::new();

    let id_0 = id_struct.retain();
    id_struct.retain();
    id_struct.release(id_0);

    id_struct.clear();

    assert_eq!(id_struct.count(), 0);

    let actual: Vec<_> = id_struct.into_iter().collect();
    assert_eq!(actual, vec![]);

    // After clearing, allocation starts fresh from 0.
    assert_eq!(id_struct.peek_next_fresh(), u32_id!(MTest; 0));
    assert_eq!(id_struct.retain(), u32_id!(MTest; 0));
}
