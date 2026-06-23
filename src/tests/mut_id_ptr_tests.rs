use crate::{IdPtr, MutIdPtr, id_ptr, mut_id_ptr, tests::util::MTest};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    mem::transmute,
    ptr::null,
};

#[test]
fn cast_to_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(MTest; ptr);

    let actual: MutIdPtr<MTest, u8> = id_ptr.cast_to::<u8>();
    let expected = mut_id_ptr!(MTest; ptr as *mut u8);
    assert_eq!(actual, expected);
}

#[test]
fn deref_ptr_test() {
    unsafe {
        let ptr: *mut _ = &mut 1;
        let id_ptr = mut_id_ptr!(MTest; ptr);

        let actual: &i32 = id_ptr.deref_ptr();
        let expected = &*ptr;
        assert_eq!(actual, expected);
    }
}

#[test]
fn deref_ptr_mut_test() {
    unsafe {
        let ptr: *mut _ = &mut 1;
        let id_ptr = mut_id_ptr!(MTest; ptr);

        let actual: &mut i32 = id_ptr.deref_ptr_mut();
        *actual = 2;
        let expected = &*ptr;
        assert_eq!(actual, expected);
    }
}

#[test]
fn from_mut_ptr_test() {
    let ptr: *mut _ = &mut 1;

    let actual: MutIdPtr<MTest, i32> = MutIdPtr::from_mut_ptr(ptr);
    let expected = mut_id_ptr!(MTest; ptr);
    assert_eq!(actual, expected);
}

#[test]
fn read_test() {
    unsafe {
        let ptr: *mut _ = &mut 1;
        let id_ptr = mut_id_ptr!(MTest; ptr);

        let actual: i32 = id_ptr.read();
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

#[test]
fn read_unaligned_test() {
    unsafe {
        let ptr: *mut _ = &mut 1;
        let id_ptr = mut_id_ptr!(MTest; ptr);

        let actual: i32 = id_ptr.read_unaligned();
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

#[test]
fn to_id_ptr_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(MTest; ptr);

    let actual: IdPtr<MTest, i32> = id_ptr.to_id_ptr();
    let expected = id_ptr!(MTest; ptr);
    assert_eq!(actual, expected);
}

#[test]
fn to_mut_ptr_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(MTest; ptr);

    let actual: *mut i32 = id_ptr.to_mut_ptr();
    let expected = ptr;
    assert_eq!(actual, expected);
}

#[test]
fn write_test() {
    unsafe {
        let ptr: *mut _ = &mut 1;
        let id_ptr = mut_id_ptr!(MTest; ptr);
        id_ptr.write(2);

        let actual: i32 = id_ptr.read();
        let expected = 2;
        assert_eq!(actual, expected);
    }
}

#[test]
fn write_unaligned_test() {
    unsafe {
        let ptr: *mut _ = &mut 1;
        let id_ptr = mut_id_ptr!(MTest; ptr);
        id_ptr.write_unaligned(2);

        let actual: i32 = id_ptr.read();
        let expected = 2;
        assert_eq!(actual, expected);
    }
}

#[test]
#[allow(clippy::clone_on_copy)]
fn clone_test() {
    let ptr: *mut _ = &mut 1;
    let id_ptr = mut_id_ptr!(MTest; ptr);

    let actual: MutIdPtr<MTest, i32> = id_ptr.clone();
    let expected = mut_id_ptr!(MTest; ptr);
    assert_eq!(actual, expected);
}

#[test]
fn debug_fmt_test() {
    let ptr: *mut i32 = unsafe { transmute(null::<i32>()) };
    let id_ptr = mut_id_ptr!(MTest; ptr);

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
fn from_from_mut_ptr_test() {
    let ptr: *mut _ = &mut 1;

    let actual: MutIdPtr<MTest, i32> = MutIdPtr::from(ptr);
    let expected = mut_id_ptr!(MTest; ptr);
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
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: Ordering = id_ptr_0.cmp(&id_ptr_0);
    let expected = ptr_0.cmp(&ptr_0);
    assert_eq!(actual, expected);

    let actual: Ordering = id_ptr_0.cmp(&id_ptr_1);
    let expected = ptr_0.cmp(&ptr_1);
    assert_eq!(actual, expected);

    let actual: Ordering = id_ptr_1.cmp(&id_ptr_0);
    let expected = ptr_1.cmp(&ptr_0);
    assert_eq!(actual, expected);
}

#[test]
fn max_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: MutIdPtr<MTest, i32> = id_ptr_0.max(id_ptr_1);
    let expected = ptr_0.max(ptr_1);
    assert_eq!(actual.to_mut_ptr(), expected);
}

#[test]
fn min_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: MutIdPtr<MTest, i32> = id_ptr_0.min(id_ptr_1);
    let expected = ptr_0.min(ptr_1);
    assert_eq!(actual.to_mut_ptr(), expected);
}

#[test]
fn clamp_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;
    let ptr_2: *mut _ = &mut 2;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);
    let id_ptr_2 = mut_id_ptr!(MTest; ptr_2);

    let id_min = id_ptr_1.min(id_ptr_2);
    let id_max = id_ptr_1.max(id_ptr_2);

    let min = ptr_1.min(ptr_2);
    let max = ptr_1.max(ptr_2);

    let actual: MutIdPtr<MTest, i32> = id_ptr_0.clamp(id_min, id_max);
    let expected = ptr_0.clamp(min, max);
    assert_eq!(actual.to_mut_ptr(), expected);

    let id_min = id_ptr_0.min(id_ptr_1);
    let id_max = id_ptr_0.max(id_ptr_1);

    let min = ptr_0.min(ptr_1);
    let max = ptr_0.max(ptr_1);

    let actual: MutIdPtr<MTest, i32> = id_ptr_2.clamp(id_min, id_max);
    let expected = ptr_2.clamp(min, max);
    assert_eq!(actual.to_mut_ptr(), expected);
}

#[test]
fn eq_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0.eq(&id_ptr_0);
    let expected = ptr_0.eq(&ptr_0);
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0.eq(&id_ptr_1);
    let expected = ptr_0.eq(&ptr_1);
    assert_eq!(actual, expected);
}

#[test]
fn ne_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0.ne(&id_ptr_0);
    let expected = ptr_0.ne(&ptr_0);
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0.ne(&id_ptr_1);
    let expected = ptr_0.ne(&ptr_1);
    assert_eq!(actual, expected);
}

#[test]
fn partial_cmp_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: Option<Ordering> = id_ptr_0.partial_cmp(&id_ptr_0);
    let expected = ptr_0.partial_cmp(&ptr_0);
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_ptr_0.partial_cmp(&id_ptr_1);
    let expected = ptr_0.partial_cmp(&ptr_1);
    assert_eq!(actual, expected);

    let actual: Option<Ordering> = id_ptr_1.partial_cmp(&id_ptr_0);
    let expected = ptr_1.partial_cmp(&ptr_0);
    assert_eq!(actual, expected);
}

#[test]
fn lt_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0 < id_ptr_0;
    let expected = ptr_0 < ptr_0;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0 < id_ptr_1;
    let expected = ptr_0 < ptr_1;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_1 < id_ptr_0;
    let expected = ptr_1 < ptr_0;
    assert_eq!(actual, expected);
}

#[test]
fn le_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0 <= id_ptr_0;
    let expected = ptr_0 <= ptr_0;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0 <= id_ptr_1;
    let expected = ptr_0 <= ptr_1;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_1 <= id_ptr_0;
    let expected = ptr_1 <= ptr_0;
    assert_eq!(actual, expected);
}

#[test]
fn gt_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0 > id_ptr_0;
    let expected = ptr_0 > ptr_0;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0 > id_ptr_1;
    let expected = ptr_0 > ptr_1;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_1 > id_ptr_0;
    let expected = ptr_1 > ptr_0;
    assert_eq!(actual, expected);
}

#[test]
fn ge_test() {
    let ptr_0: *mut _ = &mut 0;
    let ptr_1: *mut _ = &mut 1;

    let id_ptr_0 = mut_id_ptr!(MTest; ptr_0);
    let id_ptr_1 = mut_id_ptr!(MTest; ptr_1);

    let actual: bool = id_ptr_0 >= id_ptr_0;
    let expected = ptr_0 >= ptr_0;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_0 >= id_ptr_1;
    let expected = ptr_0 >= ptr_1;
    assert_eq!(actual, expected);

    let actual: bool = id_ptr_1 >= id_ptr_0;
    let expected = ptr_1 >= ptr_0;
    assert_eq!(actual, expected);
}

#[test]
fn pointer_fmt_test() {
    let ptr: *mut i32 = unsafe { transmute(null::<i32>()) };
    let id_ptr = mut_id_ptr!(MTest; ptr);

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
