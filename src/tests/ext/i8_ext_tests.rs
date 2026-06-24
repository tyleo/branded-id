use crate::{I8Id, ext::I8Ext, i8_id as id, tests::util::BTest};

#[test]
fn to_i8_id_test() {
    let i8 = 1i8;

    let actual: I8Id<BTest> = i8.to_i8_id();
    let expected = id!(i8);
    assert_eq!(actual, expected);
}
