use crate::{id_ptr, isize_id, mut_id_ptr, tests::util::MTest, IdPtr};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ptr::null,
};

#[test]
fn deref_ptr_test() {
    unsafe {
        let ptr: *const _ = &1;
        let id_ptr = id_ptr!(MTest; ptr);

        let actual: &i32 = id_ptr.deref_ptr();
        let expected = &*ptr;
        assert_eq!(actual, expected);
    }
}

#[test]
fn from_ptr_test() {
    let ptr: *const _ = &1;

    let actual: IdPtr<MTest, i32> = IdPtr::from_ptr(ptr);
    let expected = id_ptr!(MTest; &1i32);
    assert_eq!(actual, expected);
}

#[test]
fn offset_test() {
    unsafe {
        let slice = [1, 2].as_slice();

        let id_ptr = id_ptr!(MTest; slice.as_ptr());
        let id_ptr = id_ptr.offset(isize_id!(1));

        let ptr = slice.as_ptr();
        let ptr = ptr.offset(1);

        let actual: &i32 = id_ptr.deref_ptr();
        let expected = &*ptr;
        assert_eq!(actual, expected);
    }
}

#[test]
fn to_ptr_test() {
    let ptr: *const _ = &1;
    let id_ptr = id_ptr!(MTest; ptr);

    let actual: *const i32 = id_ptr.to_ptr();
    let expected = ptr;
    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let ptr: *const _ = &1;
    let id_ptr = id_ptr!(MTest; ptr);

    let actual: IdPtr<MTest, i32> = id_ptr.clone();
    let expected = id_ptr!(MTest; ptr);
    assert_eq!(actual, expected);
}

#[test]
fn debug_fmt_test() {
    let ptr: *const _ = null::<i32>();
    let id_ptr = id_ptr!(MTest; ptr);

    let actual: String = format!("{:?}", id_ptr);
    let expected = "MTest(0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+?}", id_ptr);
    let expected = "MTest(+0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-?}", id_ptr);
    let expected = "MTest(0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#?}", id_ptr);
    let expected = "id_sys::tests::util::m_test::MTest(0x0000000000000000)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25?}", id_ptr);
    let expected = "MTest(                      0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25?}", id_ptr);
    let expected = "MTest(0x0                      )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25?}", id_ptr);
    let expected = "MTest(                      0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25?}", id_ptr);
    let expected = "MTest(           0x0           )";
    assert_eq!(actual, expected);
}

#[test]
fn from_const_ptr_test() {
    let ptr: *const _ = &1;

    let actual: IdPtr<MTest, i32> = IdPtr::from(ptr);
    let expected = id_ptr!(MTest; &1i32);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_ptr_test() {
    let ptr: *mut _ = &mut 1;

    let actual: IdPtr<MTest, i32> = IdPtr::from(ptr);
    let expected = id_ptr!(MTest; ptr);
    assert_eq!(actual, expected);
}

#[test]
fn from_mut_id_ptr_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(MTest; ptr);

    let actual: IdPtr<MTest, i32> = IdPtr::from(id_ptr);
    let expected = id_ptr!(MTest; ptr);
    assert_eq!(actual, expected);
}

#[test]
fn hash_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(MTest; ptr);
    let mut hasher_0 = DefaultHasher::new();
    id_ptr.hash(&mut hasher_0);

    let mut hasher_1 = DefaultHasher::new();
    ptr.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn hash_slice_test() {
    let ptr_0: *mut _ = &mut 1;
    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let ptr_1: *mut _ = &mut 1;
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let ids = [id_ptr_0, id_ptr_1];
    let mut hasher_0 = DefaultHasher::new();
    ids.hash(&mut hasher_0);

    let ints = [ptr_0, ptr_1];
    let mut hasher_1 = DefaultHasher::new();
    ints.hash(&mut hasher_1);

    let actual: u64 = hasher_0.finish();
    let expected = hasher_1.finish();
    assert_eq!(actual, expected);
}

#[test]
fn cmp_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: Ordering = id_ptr_0.cmp(&id_ptr_0);
    let expected = Ordering::Equal;
    assert_eq!(actual, expected);

    let actual: Ordering = id_ptr_0.cmp(&id_ptr_1);
    let expected = Ordering::Less;
    assert_eq!(actual, expected);

    let actual: Ordering = id_ptr_1.cmp(&id_ptr_0);
    let expected = Ordering::Greater;
    assert_eq!(actual, expected);
}

#[test]
fn max_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: IdPtr<MTest, i32> = id_ptr_0.max(id_ptr_1);
    let expected = id_ptr_1;
    assert_eq!(actual, expected);
}

#[test]
fn min_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: IdPtr<MTest, i32> = id_ptr_0.min(id_ptr_1);
    let expected = id_ptr_0;
    assert_eq!(actual, expected);
}

#[test]
fn clamp_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;
    let ptr_2 = 3 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);
    let id_ptr_2 = id_ptr!(MTest; ptr_2);

    let actual: IdPtr<MTest, i32> = id_ptr_0.clamp(id_ptr_1, id_ptr_2);
    let expected = id_ptr_1;
    assert_eq!(actual, expected);

    let actual: IdPtr<MTest, i32> = id_ptr_2.clamp(id_ptr_0, id_ptr_1);
    let expected = id_ptr_1;
    assert_eq!(actual, expected);
}

#[test]
fn eq_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0.eq(&id_ptr_0);
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0.eq(&id_ptr_1);
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn ne_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0.ne(&id_ptr_0);
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0.ne(&id_ptr_1);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: Option<Ordering> = id_ptr_0.partial_cmp(&id_ptr_0);
    let expected = Some(Ordering::Equal);
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_ptr_0.partial_cmp(&id_ptr_1);
    let expected = Some(Ordering::Less);
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_ptr_1.partial_cmp(&id_ptr_0);
    let expected = Some(Ordering::Greater);
    assert_eq!(actual, expected);
}

#[test]
fn lt_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0 < id_ptr_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0 < id_ptr_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_1 < id_ptr_0;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn le_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0 <= id_ptr_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0 <= id_ptr_1;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_1 <= id_ptr_0;
    let expected = false;
    assert_eq!(actual, expected);
}

#[test]
fn gt_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0 > id_ptr_0;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0 > id_ptr_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_1 > id_ptr_0;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn ge_test() {
    let ptr_0 = 1 as *const i32;
    let ptr_1 = 2 as *const i32;

    let id_ptr_0 = id_ptr!(MTest; ptr_0);
    let id_ptr_1 = id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0 >= id_ptr_0;
    let expected = true;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0 >= id_ptr_1;
    let expected = false;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_1 >= id_ptr_0;
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn pointer_fmt_test() {
    let ptr: *const _ = null::<i32>();
    let id_ptr = id_ptr!(MTest; ptr);

    let actual: String = format!("{:p}", id_ptr);
    let expected = "MTest(0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:+p}", id_ptr);
    let expected = "MTest(+0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:-p}", id_ptr);
    let expected = "MTest(0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:#p}", id_ptr);
    let expected = "id_sys::tests::util::m_test::MTest(0x0000000000000000)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:25p}", id_ptr);
    let expected = "MTest(                      0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:<25p}", id_ptr);
    let expected = "MTest(0x0                      )";
    assert_eq!(actual, expected);

    let actual: String = format!("{:>25p}", id_ptr);
    let expected = "MTest(                      0x0)";
    assert_eq!(actual, expected);

    let actual: String = format!("{:^25p}", id_ptr);
    let expected = "MTest(           0x0           )";
    assert_eq!(actual, expected);
}
