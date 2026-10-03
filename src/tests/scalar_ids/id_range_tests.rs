use crate::{IdRange, U8Id, U32Id, tests::util::BTest, u32_id as id};

#[test]
fn clone_test() {
    let mut range = id!(BTest; 3).range_from_zero();
    range.next();

    let actual: Vec<U32Id<BTest>> = range.clone().collect();
    let expected = vec![id!(BTest; 1), id!(BTest; 2)];
    assert_eq!(actual, expected);

    assert_eq!(range.next(), Some(id!(BTest; 1)));
}

#[test]
fn debug_fmt_test() {
    let range: IdRange<U32Id<BTest>> = id!(BTest; 3).range_from_zero();

    let actual = format!("{range:?}");
    let expected = "IdRange(0..3)";
    assert_eq!(actual, expected);
}

#[test]
fn next_test() {
    let mut range = id!(BTest; 2).range_from_zero();

    assert_eq!(range.next(), Some(id!(BTest; 0)));
    assert_eq!(range.next(), Some(id!(BTest; 1)));
    assert_eq!(range.next(), None);
    assert_eq!(range.next(), None);
}

#[test]
fn next_back_test() {
    let mut range = id!(BTest; 3).range_from_zero();

    assert_eq!(range.next_back(), Some(id!(BTest; 2)));
    assert_eq!(range.next(), Some(id!(BTest; 0)));
    assert_eq!(range.next_back(), Some(id!(BTest; 1)));
    assert_eq!(range.next_back(), None);
}

#[test]
fn len_test() {
    let mut range = id!(BTest; 3).range_from_zero();
    assert_eq!(range.len(), 3);

    range.next();
    assert_eq!(range.len(), 2);
}

#[test]
fn range_from_zero_of_zero_is_empty_test() {
    let mut range = id!(BTest; 0).range_from_zero();

    assert_eq!(range.len(), 0);
    assert_eq!(range.next(), None);
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
fn from_len_fills_the_id_width_test() {
    let mut range = IdRange::<U8Id<BTest>>::from_len(256);

    assert_eq!(range.next_back(), Some(U8Id::from_u8(255)));
}

#[test]
#[should_panic(expected = "an id range's last id fits its id width")]
fn from_len_panics_past_the_id_width_test() {
    IdRange::<U8Id<BTest>>::from_len(257);
}
