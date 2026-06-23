/// Builds a [`MutIdPtr`](crate::MutIdPtr) from a raw pointer. Forms: `mut_id_ptr!(ptr)`,
/// `mut_id_ptr!(Brand; ptr)`, and `mut_id_ptr!(Brand; Value; ptr)`.
///
/// # Examples
/// ```rust
/// use branded_id::{mut_id_ptr, MutIdPtr};
/// struct BRow;
/// let mut x = 5i32;
/// let p: MutIdPtr<BRow, i32> = mut_id_ptr!(BRow; &mut x as *mut i32);
/// assert_eq!(unsafe { *p.to_mut_ptr() }, 5);
/// ```
#[macro_export]
macro_rules! mut_id_ptr {
    ($ptr:expr) => {
        $crate::MutIdPtr::<_, _>::from_mut_ptr($ptr)
    };
    ($brand:ty; $ptr:expr) => {
        $crate::MutIdPtr::<$brand, _>::from_mut_ptr($ptr)
    };
    ($brand:ty; $value:ty; $ptr:expr) => {
        $crate::MutIdPtr::<$brand, $value>::from_mut_ptr($ptr)
    };
}
