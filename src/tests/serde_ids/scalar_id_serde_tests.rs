use crate::{U32Id, UsizeId, i64_id, tests::util::BTest, u32_id, usize_id};
use serde_test::{Token, assert_de_tokens_error, assert_tokens};
use std::collections::BTreeMap;

#[test]
fn a_scalar_id_round_trips_as_its_integer() {
    let id = u32_id!(BTest; 7);

    assert_tokens(&id, &[Token::U32(7)]);
}

#[test]
fn a_negative_id_round_trips() {
    let id = i64_id!(BTest; -3);

    assert_tokens(&id, &[Token::I64(-3)]);
}

#[test]
fn a_scalar_id_works_as_a_map_key() {
    let map: BTreeMap<UsizeId<BTest>, &str> =
        BTreeMap::from([(usize_id!(BTest; 2), "b"), (usize_id!(BTest; 1), "a")]);

    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(2) },
            Token::U64(1),
            Token::BorrowedStr("a"),
            Token::U64(2),
            Token::BorrowedStr("b"),
            Token::MapEnd,
        ],
    );
}

#[test]
fn a_value_past_the_width_errors() {
    assert_de_tokens_error::<U32Id<BTest>>(
        &[Token::U64(4_294_967_296)],
        "invalid value: integer `4294967296`, expected u32",
    );
}
