use crate::{I16Id, ext::I16Ext, i16_id as id, tests::util::BTest};

#[test]
fn to_i16_id_test() {
    let i16 = 1i16;

    let actual: I16Id<BTest> = i16.to_i16_id();
    let expected = id!(i16);
    assert_eq!(actual, expected);
}
