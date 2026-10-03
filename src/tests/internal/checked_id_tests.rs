use crate::{I8Id, U8Id, UsizeId, internal::checked_id, tests::util::BTest, u8_id, usize_id};

#[test]
fn checked_id_test() {
    let actual: Option<U8Id<BTest>> = checked_id(255);
    let expected = Some(u8_id!(BTest; 255));
    assert_eq!(actual, expected);
}

#[test]
fn checked_id_past_the_width_test() {
    let actual: Option<U8Id<BTest>> = checked_id(256);
    assert_eq!(actual, None);
}

#[test]
fn checked_id_past_a_signed_width_test() {
    let actual: Option<I8Id<BTest>> = checked_id(128);
    assert_eq!(actual, None);
}

#[test]
fn checked_id_of_usize_max_test() {
    let actual: Option<UsizeId<BTest>> = checked_id(usize::MAX);
    let expected = Some(usize_id!(BTest; usize::MAX));
    assert_eq!(actual, expected);
}
