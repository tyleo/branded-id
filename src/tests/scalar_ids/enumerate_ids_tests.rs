use crate::{U8Id, U32Id, ext::IteratorExt, tests::util::BTest, u32_id as id};

#[test]
fn clone_test() {
    let mut ids = ["a", "b", "c"].into_iter().enumerate_ids::<U32Id<BTest>>();
    ids.next();

    let actual: Vec<_> = ids.clone().collect();
    let expected = vec![(id!(BTest; 1), "b"), (id!(BTest; 2), "c")];
    assert_eq!(actual, expected);

    assert_eq!(ids.next(), Some((id!(BTest; 1), "b")));
}

#[test]
fn debug_fmt_test() {
    let ids = [1, 2].into_iter().enumerate_ids::<U32Id<BTest>>();

    let actual = format!("{ids:?}");
    let expected = "EnumerateIds { iter: IntoIter([1, 2]), count: 0 }";
    assert_eq!(actual, expected);
}

#[test]
fn next_test() {
    let mut ids = ["a", "b"].into_iter().enumerate_ids::<U32Id<BTest>>();

    assert_eq!(ids.next(), Some((id!(BTest; 0), "a")));
    assert_eq!(ids.next(), Some((id!(BTest; 1), "b")));
    assert_eq!(ids.next(), None);
}

#[test]
fn next_back_test() {
    let mut ids = ["a", "b", "c"].into_iter().enumerate_ids::<U32Id<BTest>>();

    assert_eq!(ids.next_back(), Some((id!(BTest; 2), "c")));
    assert_eq!(ids.next(), Some((id!(BTest; 0), "a")));
    assert_eq!(ids.next_back(), Some((id!(BTest; 1), "b")));
    assert_eq!(ids.next_back(), None);
}

#[test]
fn len_test() {
    let mut ids = ["a", "b", "c"].into_iter().enumerate_ids::<U32Id<BTest>>();
    assert_eq!(ids.len(), 3);

    ids.next();
    assert_eq!(ids.len(), 2);
}

#[test]
fn fills_the_id_width_test() {
    let ids: Vec<_> = (0..256).enumerate_ids::<U8Id<BTest>>().collect();

    assert_eq!(ids.last(), Some(&(U8Id::from_u8(255), 255)));
}

#[test]
#[should_panic(expected = "an enumerated position fits the id width")]
fn panics_past_the_id_width_test() {
    (0..257).enumerate_ids::<U8Id<BTest>>().for_each(drop);
}

#[test]
#[should_panic(expected = "an enumerated position fits the id width")]
fn next_back_panics_past_the_id_width_test() {
    (0..257).enumerate_ids::<U8Id<BTest>>().next_back();
}
