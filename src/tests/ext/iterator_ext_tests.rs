use crate::{U32Id, ext::IteratorExt, tests::util::BTest, u32_id as id};

#[test]
fn enumerate_ids_test() {
    let actual: Vec<(U32Id<BTest>, &str)> = ["a", "b"].into_iter().enumerate_ids().collect();
    let expected = vec![(id!(BTest; 0), "a"), (id!(BTest; 1), "b")];
    assert_eq!(actual, expected);
}
