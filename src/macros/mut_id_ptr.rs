/// Builds a [`MutIdPtr`](crate::MutIdPtr) from a raw pointer. Forms: `mut_id_ptr!(ptr)`,
/// `mut_id_ptr!(Brand; ptr)`, and `mut_id_ptr!(Brand; Value; ptr)`.
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
