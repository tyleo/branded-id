use crate::{
    id::{tests::marker::MUnknown, Id32},
    id32,
};

#[test]
fn from_test() {
    let id = id32!(MUnknown; 1);

    let actual: u32 = From::from(id);
    let expected = 1;
    assert_eq!(actual, expected);
}
