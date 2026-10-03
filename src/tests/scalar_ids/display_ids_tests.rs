use crate::{SliceExt, U32Id, tests::util::BTest, u32_id as id};

#[test]
fn debug_fmt_test() {
    let ids = [id!(BTest; 1), id!(BTest; 2)];

    let actual = format!("{:?}", ids.display_ids());
    let expected = "[BTest(1), BTest(2)]";
    assert_eq!(actual, expected);
}

#[test]
fn display_fmt_test() {
    let ids = [id!(BTest; 1), id!(BTest; 2)];

    let actual = format!("{}", ids.display_ids());
    let expected = "[1, 2]";
    assert_eq!(actual, expected);
}

#[test]
fn display_fmt_empty_test() {
    let ids: [U32Id<BTest>; 0] = [];

    let actual = format!("{}", ids.display_ids());
    let expected = "[]";
    assert_eq!(actual, expected);
}

#[test]
fn display_fmt_flags_test() {
    let ids = [id!(BTest; 1), id!(BTest; 20)];

    let actual = format!("{:>3}", ids.display_ids());
    let expected = "[  1,  20]";
    assert_eq!(actual, expected);
}

#[test]
fn display_fmt_matches_integer_debug_test() {
    let ids = Vec::from([id!(BTest; 3), id!(BTest; 0), id!(BTest; 7)]);

    let actual = format!("{}", ids.display_ids());
    let expected = format!("{:?}", [3, 0, 7]);
    assert_eq!(actual, expected);
}
