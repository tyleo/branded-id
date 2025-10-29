use crate::{
    i32_id, isize_id, tests::util::MTest, u32_id as id, usize_id, I32Id, IsizeId, U32Id, UsizeId,
};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::FromStr,
};

#[test]
fn from_u32_test() {
    let actual: U32Id<MTest> = U32Id::<MTest>::from_u32(1);
    let expected = id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_i32_id_test() {
    let id = id!(MTest; 1);

    let actual: I32Id<MTest> = id.to_i32_id();
    let expected = i32_id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_isize_id_test() {
    let id = id!(MTest; 1);

    let actual: IsizeId<MTest> = id.to_isize_id();
    let expected = isize_id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_u32_test() {
    let id = id!(MTest; 1);

    let actual: u32 = id.to_u32();
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn to_usize_id_test() {
    let id = id!(MTest; 1);

    let actual: UsizeId<MTest> = id.to_usize_id();
    let expected = usize_id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn binary_fmt_test() {
    let id = id!(MTest; 2);

    let actual: String = format!("{:b}", id);
    let expected = "MTest(10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+b}", id);
    let expected = "MTest(+10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-b}", id);
    let expected = "MTest(10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#b}", id);
    let expected = "id_sys::tests::util::m_test::MTest(0b10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25b}", id);
    let expected = "MTest(                       10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25b}", id);
    let expected = "MTest(10                       )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25b}", id);
    let expected = "MTest(                       10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25b}", id);
    let expected = "MTest(           10            )";
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let id = id!(MTest; 1);

    let actual: U32Id<MTest> = id.clone();
    let expected = id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn debug_fmt_test() {
    let id = id!(MTest; 1);

    let actual: String = format!("{:?}", id);
    let expected = "MTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+?}", id);
    let expected = "MTest(+1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-?}", id);
    let expected = "MTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = "id_sys::tests::util::m_test::MTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25?}", id);
    let expected = "MTest(                        1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25?}", id);
    let expected = "MTest(1                        )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25?}", id);
    let expected = "MTest(                        1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25?}", id);
    let expected = "MTest(            1            )";
    assert_eq!(actual, expected);
}

#[test]
fn display_fmt_test() {
    let id = id!(MTest; 1);

    let actual: String = format!("{}", id);
    let expected = "MTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+}", id);
    let expected = "MTest(+1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-}", id);
    let expected = "MTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#}", id);
    let expected = "id_sys::tests::util::m_test::MTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25}", id);
    let expected = "MTest(                        1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25}", id);
    let expected = "MTest(1                        )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25}", id);
    let expected = "MTest(                        1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25}", id);
    let expected = "MTest(            1            )";
    assert_eq!(actual, expected);
}

#[test]
fn from_test() {
    let actual: U32Id<MTest> = From::from(1);
    let expected = id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn from_str_test() {
    let actual: U32Id<MTest> = <U32Id<MTest> as FromStr>::from_str("1").unwrap();
    let expected = id!(MTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn hash_test() {
    let id = id!(MTest; 1);
    let mut hasher_0 = DefaultHasher::new();
    id.hash(&mut hasher_0);

    let int = 1u32;
    let mut hasher_1 = DefaultHasher::new();
    int.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn hash_slice_test() {
    let ids = [id!(MTest; 1), id!(MTest; 2)];
    let mut hasher_0 = DefaultHasher::new();
    ids.hash(&mut hasher_0);

    let ints = [1u32, 2u32];
    let mut hasher_1 = DefaultHasher::new();
    ints.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn lower_exp_fmt_test() {
    let id = id!(MTest; 2);

    let actual: String = format!("{:e}", id);
    let expected = "MTest(2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+e}", id);
    let expected = "MTest(+2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-e}", id);
    let expected = "MTest(2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#e}", id);
    let expected = "id_sys::tests::util::m_test::MTest(2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25e}", id);
    let expected = "MTest(                      2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25e}", id);
    let expected = "MTest(2e0                      )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25e}", id);
    let expected = "MTest(                      2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25e}", id);
    let expected = "MTest(           2e0           )";
    assert_eq!(actual, expected);
}

#[test]
fn lower_hex_fmt_test() {
    let id = id!(MTest; 10);

    let actual: String = format!("{:x}", id);
    let expected = "MTest(a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+x}", id);
    let expected = "MTest(+a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-x}", id);
    let expected = "MTest(a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#x}", id);
    let expected = "id_sys::tests::util::m_test::MTest(0xa)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25x}", id);
    let expected = "MTest(                        a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25x}", id);
    let expected = "MTest(a                        )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25x}", id);
    let expected = "MTest(                        a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25x}", id);
    let expected = "MTest(            a            )";
    assert_eq!(actual, expected);
}

#[test]
fn octal_fmt_test() {
    let id = id!(MTest; 10);

    let actual: String = format!("{:o}", id);
    let expected = "MTest(12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+o}", id);
    let expected = "MTest(+12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-o}", id);
    let expected = "MTest(12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#o}", id);
    let expected = "id_sys::tests::util::m_test::MTest(0o12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25o}", id);
    let expected = "MTest(                       12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25o}", id);
    let expected = "MTest(12                       )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25o}", id);
    let expected = "MTest(                       12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25o}", id);
    let expected = "MTest(           12            )";
    assert_eq!(actual, expected);
}

#[test]
fn cmp_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: Ordering = id_0.cmp(&id_0);
    let expected = Ordering::Equal;
    assert_eq!(actual, expected);

    let actual: Ordering = id_0.cmp(&id_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Ordering = id_1.cmp(&id_0);
    let expected = Ordering::Greater;
    assert_eq!(actual, expected);
}

#[test]
fn max_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: U32Id<MTest> = id_0.max(id_1);
    let expected = id!(2);
    assert_eq!(actual, expected);
}

#[test]
fn min_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: U32Id<MTest> = id_0.min(id_1);
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn clamp_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);
    let id_2 = id!(MTest; 3);

    let actual: U32Id<MTest> = id_0.clamp(id_1, id_2);
    let expected = id!(2);
    assert_eq!(actual, expected);

    let actual: U32Id<MTest> = id_2.clamp(id_0, id_1);
    let expected = id!(2);
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: bool = id_0.eq(&id_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0.eq(&id_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: bool = id_0.ne(&id_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_0.ne(&id_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: Option<Ordering> = id_0.partial_cmp(&id_0);
    let expected = Some(Ordering::Equal);
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_0.partial_cmp(&id_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_1.partial_cmp(&id_0);
    let expected = Some(Ordering::Greater);
    assert_eq!(actual, expected);
}

#[test]
fn lt_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: bool = id_0 < id_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_0 < id_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_1 < id_0;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn le_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: bool = id_0 <= id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 <= id_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_1 <= id_0;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn gt_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: bool = id_0 > id_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_0 > id_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_1 > id_0;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn ge_test() {
    let id_0 = id!(MTest; 1);
    let id_1 = id!(MTest; 2);

    let actual: bool = id_0 >= id_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0 >= id_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_1 >= id_0;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn upper_exp_fmt_test() {
    let id = id!(MTest; 2);

    let actual: String = format!("{:E}", id);
    let expected = "MTest(2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+E}", id);
    let expected = "MTest(+2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-E}", id);
    let expected = "MTest(2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#E}", id);
    let expected = "id_sys::tests::util::m_test::MTest(2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25E}", id);
    let expected = "MTest(                      2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25E}", id);
    let expected = "MTest(2E0                      )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25E}", id);
    let expected = "MTest(                      2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25E}", id);
    let expected = "MTest(           2E0           )";
    assert_eq!(actual, expected);
}

#[test]
fn upper_hex_fmt_test() {
    let id = id!(MTest; 10);

    let actual: String = format!("{:X}", id);
    let expected = "MTest(A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+X}", id);
    let expected = "MTest(+A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-X}", id);
    let expected = "MTest(A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#X}", id);
    let expected = "id_sys::tests::util::m_test::MTest(0xA)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25X}", id);
    let expected = "MTest(                        A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25X}", id);
    let expected = "MTest(A                        )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25X}", id);
    let expected = "MTest(                        A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25X}", id);
    let expected = "MTest(            A            )";
    assert_eq!(actual, expected);
}
