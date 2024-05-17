use crate::{soa::UsizeIdStruct, tests::util::MTest, usize_id};

#[test]
fn new_test() {
    let id_struct = UsizeIdStruct::<MTest>::new();

    let actual = id_struct.count();
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn count_test() {
    let mut id_struct = UsizeIdStruct::<MTest>::new();

    let actual = id_struct.count();
    let expected = 0;
    assert_eq!(actual, expected);

    let id_0 = id_struct.retain();
    let id_1 = id_struct.retain();

    let actual = id_struct.count();
    let expected = 2;
    assert_eq!(actual, expected);

    id_struct.release(id_0);

    let actual = id_struct.count();
    let expected = 1;
    assert_eq!(actual, expected);

    id_struct.release(id_1);

    let actual = id_struct.count();
    let expected = 0;
    assert_eq!(actual, expected);
}

#[test]
fn retain_release_test() {
    let mut id_struct = UsizeIdStruct::<MTest>::new();

    let id_0 = id_struct.retain();

    let actual = id_0;
    let expected = usize_id!(MTest; 0);
    assert_eq!(actual, expected);

    let id_1 = id_struct.retain();

    let actual = id_1;
    let expected = usize_id!(MTest; 1);
    assert_eq!(actual, expected);

    id_struct.release(id_0);

    let actual = id_struct.retain();
    let expected = usize_id!(MTest; 0);
    assert_eq!(actual, expected);

    id_struct.release(id_0);
    id_struct.release(id_1);

    let actual = id_struct.retain();
    let expected = usize_id!(MTest; 1);
    assert_eq!(actual, expected);

    let actual = id_struct.retain();
    let expected = usize_id!(MTest; 0);
    assert_eq!(actual, expected);
}
