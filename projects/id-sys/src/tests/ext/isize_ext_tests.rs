use crate::{ext::IsizeExt, isize_id as id, tests::util::MTest, IsizeId};

#[test]
fn to_isize_id_test() {
    let isize = 1isize;

    let actual: IsizeId<MTest> = isize.to_isize_id();
    let expected = id!(isize);
    assert_eq!(actual, expected);
}
