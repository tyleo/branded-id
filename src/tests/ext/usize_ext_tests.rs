use crate::{UsizeId, ext::UsizeExt, tests::util::BTest, usize_id as id};

#[test]
fn to_usize_id_test() {
    let usize = 1usize;

    let actual: UsizeId<BTest> = usize.to_usize_id();
    let expected = id!(usize);
    assert_eq!(actual, expected);
}
