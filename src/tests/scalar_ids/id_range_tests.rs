use crate::{I8Id, IdRange, RangeExt, U8Id, U32Id, i8_id, tests::util::BTest, u32_id as id};

#[test]
fn clone_test() {
    let mut range = (id!(BTest; 0)..id!(BTest; 3)).into_id_range();
    range.next();

    let actual: Vec<U32Id<BTest>> = range.clone().collect();
    let expected = vec![id!(BTest; 1), id!(BTest; 2)];
    assert_eq!(actual, expected);

    assert_eq!(range.next(), Some(id!(BTest; 1)));
}

#[test]
fn debug_fmt_test() {
    let range = (id!(BTest; 0)..id!(BTest; 3)).into_id_range();

    let actual = format!("{range:?}");
    let expected = "IdRange(0..3)";
    assert_eq!(actual, expected);
}

#[test]
fn next_test() {
    let mut range = (id!(BTest; 0)..id!(BTest; 2)).into_id_range();

    assert_eq!(range.next(), Some(id!(BTest; 0)));
    assert_eq!(range.next(), Some(id!(BTest; 1)));
    assert_eq!(range.next(), None);
    assert_eq!(range.next(), None);
}

#[test]
fn next_back_test() {
    let mut range = (id!(BTest; 0)..id!(BTest; 3)).into_id_range();

    assert_eq!(range.next_back(), Some(id!(BTest; 2)));
    assert_eq!(range.next(), Some(id!(BTest; 0)));
    assert_eq!(range.next_back(), Some(id!(BTest; 1)));
    assert_eq!(range.next_back(), None);
}

#[test]
fn len_test() {
    let mut range = (id!(BTest; 0)..id!(BTest; 3)).into_id_range();
    assert_eq!(range.len(), 3);

    range.next();
    assert_eq!(range.len(), 2);
}

#[test]
fn reversed_range_is_empty_test() {
    let mut range = (id!(BTest; 3)..id!(BTest; 1)).into_id_range();

    assert_eq!(range.len(), 0);
    assert_eq!(range.next(), None);
}

#[test]
fn negative_ids_test() {
    let range = i8_id!(BTest; -2)..i8_id!(BTest; 1);

    let actual: Vec<I8Id<BTest>> = range.into_id_range().collect();
    let expected = vec![i8_id!(BTest; -2), i8_id!(BTest; -1), i8_id!(BTest; 0)];
    assert_eq!(actual, expected);
}

#[test]
fn from_range_test() {
    let actual: Vec<U32Id<BTest>> = IdRange::from(id!(BTest; 1)..id!(BTest; 3)).collect();
    let expected = vec![id!(BTest; 1), id!(BTest; 2)];
    assert_eq!(actual, expected);
}

#[test]
fn from_len_test() {
    let actual: Vec<U32Id<BTest>> = IdRange::from_len(2).collect();
    let expected = vec![id!(BTest; 0), id!(BTest; 1)];
    assert_eq!(actual, expected);
}

#[test]
fn from_len_of_zero_is_empty_test() {
    let mut range = IdRange::<U32Id<BTest>>::from_len(0);

    assert_eq!(range.next(), None);
}

#[test]
fn from_len_reaches_the_last_id_below_the_width_end_test() {
    let mut range = IdRange::<U8Id<BTest>>::from_len(255);

    assert_eq!(range.next_back(), Some(U8Id::from_u8(254)));
}

#[test]
#[should_panic(expected = "an id range's end fits its id width")]
fn from_len_panics_at_the_width_end_test() {
    IdRange::<U8Id<BTest>>::from_len(256);
}
