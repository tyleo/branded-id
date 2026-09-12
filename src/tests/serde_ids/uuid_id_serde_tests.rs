use crate::{tests::util::BTest, uuid_id};
use serde_test::{Configure, Token, assert_tokens};
use uuid::Uuid;

#[test]
fn a_uuid_id_round_trips_as_its_uuid() {
    let id = uuid_id!(BTest; Uuid::from_u128(0x1234));

    assert_tokens(
        &id.readable(),
        &[Token::Str("00000000-0000-0000-0000-000000001234")],
    );
}
