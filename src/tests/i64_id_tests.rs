use crate::{
    I8Id, I16Id, I32Id, I64Id, I128Id, IsizeId, U8Id, U16Id, U32Id, U64Id, U128Id, UsizeId, i8_id,
    i16_id, i32_id, i64_id as id, i128_id, isize_id, tests::util::BTest, u8_id, u16_id, u32_id,
    u64_id, u128_id, usize_id,
};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::FromStr,
};

#[test]
fn from_i64_test() {
    let actual: I64Id<BTest> = I64Id::<BTest>::from_i64(1);
    let expected = id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_i64_test() {
    let id = id!(BTest; 1);

    let actual: i64 = id.to_i64();
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn to_i8_id_test() {
    let id = id!(BTest; 1);

    let actual: I8Id<BTest> = id.to_i8_id();
    let expected = i8_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_i16_id_test() {
    let id = id!(BTest; 1);

    let actual: I16Id<BTest> = id.to_i16_id();
    let expected = i16_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_i32_id_test() {
    let id = id!(BTest; 1);

    let actual: I32Id<BTest> = id.to_i32_id();
    let expected = i32_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_i128_id_test() {
    let id = id!(BTest; 1);

    let actual: I128Id<BTest> = id.to_i128_id();
    let expected = i128_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_isize_id_test() {
    let id = id!(BTest; 1);

    let actual: IsizeId<BTest> = id.to_isize_id();
    let expected = isize_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_u8_id_test() {
    let id = id!(BTest; 1);

    let actual: U8Id<BTest> = id.to_u8_id();
    let expected = u8_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_u16_id_test() {
    let id = id!(BTest; 1);

    let actual: U16Id<BTest> = id.to_u16_id();
    let expected = u16_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_u32_id_test() {
    let id = id!(BTest; 1);

    let actual: U32Id<BTest> = id.to_u32_id();
    let expected = u32_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_u64_id_test() {
    let id = id!(BTest; 1);

    let actual: U64Id<BTest> = id.to_u64_id();
    let expected = u64_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_u64_id_reinterprets_sign_test() {
    let id = id!(BTest; -1);

    let actual: U64Id<BTest> = id.to_u64_id();
    let expected = u64_id!(BTest; u64::MAX);
    assert_eq!(actual, expected);
}

#[test]
fn to_u128_id_test() {
    let id = id!(BTest; 1);

    let actual: U128Id<BTest> = id.to_u128_id();
    let expected = u128_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn to_usize_id_test() {
    let id = id!(BTest; 1);

    let actual: UsizeId<BTest> = id.to_usize_id();
    let expected = usize_id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn binary_fmt_test() {
    let id = id!(BTest; 2);

    let actual: String = format!("{:b}", id);
    let expected = "BTest(10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+b}", id);
    let expected = "BTest(+10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-b}", id);
    let expected = "BTest(10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#b}", id);
    let expected = "branded_id::tests::util::b_test::BTest(0b10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25b}", id);
    let expected = "BTest(                       10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25b}", id);
    let expected = "BTest(10                       )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25b}", id);
    let expected = "BTest(                       10)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25b}", id);
    let expected = "BTest(           10            )";
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let id = id!(BTest; 1);

    let actual: I64Id<BTest> = id.clone();
    let expected = id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn debug_fmt_test() {
    let id = id!(BTest; 1);

    let actual: String = format!("{:?}", id);
    let expected = "BTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+?}", id);
    let expected = "BTest(+1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-?}", id);
    let expected = "BTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id);
    let expected = "branded_id::tests::util::b_test::BTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25?}", id);
    let expected = "BTest(                        1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25?}", id);
    let expected = "BTest(1                        )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25?}", id);
    let expected = "BTest(                        1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25?}", id);
    let expected = "BTest(            1            )";
    assert_eq!(actual, expected);
}

#[test]
fn display_fmt_test() {
    let id = id!(BTest; 1);

    let actual: String = format!("{}", id);
    let expected = "BTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+}", id);
    let expected = "BTest(+1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-}", id);
    let expected = "BTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#}", id);
    let expected = "branded_id::tests::util::b_test::BTest(1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25}", id);
    let expected = "BTest(                        1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25}", id);
    let expected = "BTest(1                        )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25}", id);
    let expected = "BTest(                        1)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25}", id);
    let expected = "BTest(            1            )";
    assert_eq!(actual, expected);
}

#[test]
fn from_test() {
    let actual: I64Id<BTest> = From::from(1);
    let expected = id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn from_str_test() {
    let actual: I64Id<BTest> = <I64Id<BTest> as FromStr>::from_str("1").unwrap();
    let expected = id!(BTest; 1);
    assert_eq!(actual, expected);
}

#[test]
fn hash_test() {
    let id = id!(BTest; 1);
    let mut hasher_0 = DefaultHasher::new();
    id.hash(&mut hasher_0);

    let int = 1i64;
    let mut hasher_1 = DefaultHasher::new();
    int.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn hash_slice_test() {
    let ids = [id!(BTest; 1), id!(BTest; 2)];
    let mut hasher_0 = DefaultHasher::new();
    ids.hash(&mut hasher_0);

    let ints = [1i64, 2i64];
    let mut hasher_1 = DefaultHasher::new();
    ints.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn lower_exp_fmt_test() {
    let id = id!(BTest; 2);

    let actual: String = format!("{:e}", id);
    let expected = "BTest(2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+e}", id);
    let expected = "BTest(+2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-e}", id);
    let expected = "BTest(2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#e}", id);
    let expected = "branded_id::tests::util::b_test::BTest(2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25e}", id);
    let expected = "BTest(                      2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25e}", id);
    let expected = "BTest(2e0                      )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25e}", id);
    let expected = "BTest(                      2e0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25e}", id);
    let expected = "BTest(           2e0           )";
    assert_eq!(actual, expected);
}

#[test]
fn lower_hex_fmt_test() {
    let id = id!(BTest; 10);

    let actual: String = format!("{:x}", id);
    let expected = "BTest(a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+x}", id);
    let expected = "BTest(+a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-x}", id);
    let expected = "BTest(a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#x}", id);
    let expected = "branded_id::tests::util::b_test::BTest(0xa)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25x}", id);
    let expected = "BTest(                        a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25x}", id);
    let expected = "BTest(a                        )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25x}", id);
    let expected = "BTest(                        a)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25x}", id);
    let expected = "BTest(            a            )";
    assert_eq!(actual, expected);
}

#[test]
fn octal_fmt_test() {
    let id = id!(BTest; 10);

    let actual: String = format!("{:o}", id);
    let expected = "BTest(12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+o}", id);
    let expected = "BTest(+12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-o}", id);
    let expected = "BTest(12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#o}", id);
    let expected = "branded_id::tests::util::b_test::BTest(0o12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25o}", id);
    let expected = "BTest(                       12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25o}", id);
    let expected = "BTest(12                       )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25o}", id);
    let expected = "BTest(                       12)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25o}", id);
    let expected = "BTest(           12            )";
    assert_eq!(actual, expected);
}

#[test]
fn cmp_test() {
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

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
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

    let actual: I64Id<BTest> = id_0.max(id_1);
    let expected = id!(2);
    assert_eq!(actual, expected);
}

#[test]
fn min_test() {
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

    let actual: I64Id<BTest> = id_0.min(id_1);
    let expected = id!(1);
    assert_eq!(actual, expected);
}

#[test]
fn clamp_test() {
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);
    let id_2 = id!(BTest; 3);

    let actual: I64Id<BTest> = id_0.clamp(id_1, id_2);
    let expected = id!(2);
    assert_eq!(actual, expected);

    let actual: I64Id<BTest> = id_2.clamp(id_0, id_1);
    let expected = id!(2);
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

    let actual: bool = id_0.eq(&id_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_0.eq(&id_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_test() {
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

    let actual: bool = id_0.ne(&id_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_0.ne(&id_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

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
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

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
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

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
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

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
    let id_0 = id!(BTest; 1);
    let id_1 = id!(BTest; 2);

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
    let id = id!(BTest; 2);

    let actual: String = format!("{:E}", id);
    let expected = "BTest(2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+E}", id);
    let expected = "BTest(+2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-E}", id);
    let expected = "BTest(2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#E}", id);
    let expected = "branded_id::tests::util::b_test::BTest(2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25E}", id);
    let expected = "BTest(                      2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25E}", id);
    let expected = "BTest(2E0                      )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25E}", id);
    let expected = "BTest(                      2E0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25E}", id);
    let expected = "BTest(           2E0           )";
    assert_eq!(actual, expected);
}

#[test]
fn upper_hex_fmt_test() {
    let id = id!(BTest; 10);

    let actual: String = format!("{:X}", id);
    let expected = "BTest(A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+X}", id);
    let expected = "BTest(+A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-X}", id);
    let expected = "BTest(A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#X}", id);
    let expected = "branded_id::tests::util::b_test::BTest(0xA)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25X}", id);
    let expected = "BTest(                        A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25X}", id);
    let expected = "BTest(A                        )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25X}", id);
    let expected = "BTest(                        A)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25X}", id);
    let expected = "BTest(            A            )";
    assert_eq!(actual, expected);
}
