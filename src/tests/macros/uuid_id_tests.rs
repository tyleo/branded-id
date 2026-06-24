use crate::{UuidId, tests::util::BTest, uuid_id};
use uuid::Uuid;

#[test]
fn uuid_id_0_test() {
    let actual: UuidId<BTest> = uuid_id!(Uuid::from_u128(1));
    let expected = UuidId::from_uuid(Uuid::from_u128(1));
    assert_eq!(actual, expected);
}

#[test]
fn uuid_id_1_test() {
    let actual: UuidId<BTest> = uuid_id!(BTest; Uuid::from_u128(1));
    let expected = UuidId::from_uuid(Uuid::from_u128(1));
    assert_eq!(actual, expected);
}
