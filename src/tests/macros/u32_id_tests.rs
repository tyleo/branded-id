use crate::{U32Id, tests::util::MTest, u32_id};

#[test]
fn u32_id_0_test() {
    let actual: U32Id<MTest> = u32_id!(1);
    let expected = U32Id::from_u32(1);
    assert_eq!(actual, expected);
}

#[test]
fn u32_id_1_test() {
    let actual: U32Id<MTest> = u32_id!(MTest; 1);
    let expected = U32Id::from_u32(1);
    assert_eq!(actual, expected);
}
