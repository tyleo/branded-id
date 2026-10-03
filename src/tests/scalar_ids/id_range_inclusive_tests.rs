use crate::{
    I8Id, IdRangeInclusive, U8Id, U32Id, ext::RangeInclusiveExt, i8_id, tests::util::BTest,
    u32_id as id,
};

#[test]
fn clone_test() {
    let mut range = (id!(BTest; 0)..=id!(BTest; 2)).into_id_range();
    range.next();

    let actual: Vec<U32Id<BTest>> = range.clone().collect();
    let expected = vec![id!(BTest; 1), id!(BTest; 2)];
    assert_eq!(actual, expected);

    assert_eq!(range.next(), Some(id!(BTest; 1)));
}

#[test]
fn debug_fmt_test() {
    let range = (id!(BTest; 0)..=id!(BTest; 3)).into_id_range();

    let actual = format!("{range:?}");
    let expected = "IdRangeInclusive(0..=3)";
    assert_eq!(actual, expected);
}

#[test]
fn next_test() {
    let mut range = (id!(BTest; 0)..=id!(BTest; 1)).into_id_range();

    assert_eq!(range.next(), Some(id!(BTest; 0)));
    assert_eq!(range.next(), Some(id!(BTest; 1)));
    assert_eq!(range.next(), None);
    assert_eq!(range.next(), None);
}

#[test]
fn next_back_test() {
    let mut range = (id!(BTest; 0)..=id!(BTest; 2)).into_id_range();

    assert_eq!(range.next_back(), Some(id!(BTest; 2)));
    assert_eq!(range.next(), Some(id!(BTest; 0)));
    assert_eq!(range.next_back(), Some(id!(BTest; 1)));
    assert_eq!(range.next_back(), None);
}

#[test]
fn len_test() {
    let mut range = (U8Id::<BTest>::from_u8(0)..=U8Id::from_u8(2)).into_id_range();
    assert_eq!(range.len(), 3);

    range.next();
    assert_eq!(range.len(), 2);
}

#[test]
fn reversed_range_is_empty_test() {
    let mut range = (id!(BTest; 3)..=id!(BTest; 1)).into_id_range();

    assert_eq!(range.next(), None);
}

#[test]
fn reaches_the_largest_id_test() {
    let mut range = (U8Id::<BTest>::MIN..=U8Id::MAX).into_id_range();

    assert_eq!(range.len(), 256);
    assert_eq!(range.next_back(), Some(U8Id::MAX));
}

#[test]
fn negative_ids_test() {
    let range = i8_id!(BTest; -2)..=i8_id!(BTest; 0);

    let actual: Vec<I8Id<BTest>> = range.into_id_range().collect();
    let expected = vec![i8_id!(BTest; -2), i8_id!(BTest; -1), i8_id!(BTest; 0)];
    assert_eq!(actual, expected);
}

#[test]
fn from_range_inclusive_test() {
    let range = id!(BTest; 1)..=id!(BTest; 3);

    let actual: Vec<U32Id<BTest>> = IdRangeInclusive::from(range).collect();
    let expected = vec![id!(BTest; 1), id!(BTest; 2), id!(BTest; 3)];
    assert_eq!(actual, expected);
}
