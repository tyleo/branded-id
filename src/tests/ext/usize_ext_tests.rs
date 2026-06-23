use crate::{UsizeId, ext::UsizeExt, tests::util::MTest, usize_id as id};

#[test]
fn to_usize_id_test() {
    let usize = 1usize;

    let actual: UsizeId<MTest> = usize.to_usize_id();
    let expected = id!(usize);
    assert_eq!(actual, expected);
}
