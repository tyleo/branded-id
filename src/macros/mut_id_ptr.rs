/// Builds a [`MutIdPtr`](crate::MutIdPtr) from a raw pointer. Forms: `mut_id_ptr!(ptr)`,
/// `mut_id_ptr!(Marker; ptr)`, and `mut_id_ptr!(Marker; Value; ptr)`.
#[macro_export]
macro_rules! mut_id_ptr {
    ($ptr: expr) => {
        $crate::MutIdPtr::<_, _>::from_mut_ptr($ptr)
    };
    ($marker: ty; $ptr: expr) => {
        $crate::MutIdPtr::<$marker, _>::from_mut_ptr($ptr)
    };
    ($marker: ty; $value: ty; $ptr: expr) => {
        $crate::MutIdPtr::<$marker, $value>::from_mut_ptr($ptr)
    };
}
