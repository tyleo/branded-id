/// Builds an [`IdPtr`](crate::IdPtr) from a raw pointer. Forms: `id_ptr!(ptr)`,
/// `id_ptr!(Brand; ptr)`, and `id_ptr!(Brand; Value; ptr)`.
///
/// # Examples
/// ```rust
/// use branded_id::{id_ptr, IdPtr};
/// struct BRow;
/// let x = 5i32;
/// let p: IdPtr<BRow, i32> = id_ptr!(BRow; &x as *const i32);
/// assert_eq!(unsafe { *p.to_ptr() }, 5);
/// ```
#[macro_export]
macro_rules! id_ptr {
    ($ptr:expr) => {
        $crate::IdPtr::<_, _>::from_ptr($ptr)
    };
    ($brand:ty; $ptr:expr) => {
        $crate::IdPtr::<$brand, _>::from_ptr($ptr)
    };
    ($brand:ty; $value:ty; $ptr:expr) => {
        $crate::IdPtr::<$brand, $value>::from_ptr($ptr)
    };
}
