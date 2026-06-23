/// Builds an [`IdPtr`](crate::IdPtr) from a raw pointer. Forms: `id_ptr!(ptr)`,
/// `id_ptr!(Marker; ptr)`, and `id_ptr!(Marker; Value; ptr)`.
#[macro_export]
macro_rules! id_ptr {
    ($ptr: expr) => {
        $crate::IdPtr::<_, _>::from_ptr($ptr)
    };
    ($marker: ty; $ptr: expr) => {
        $crate::IdPtr::<$marker, _>::from_ptr($ptr)
    };
    ($marker: ty; $value: ty; $ptr: expr) => {
        $crate::IdPtr::<$marker, $value>::from_ptr($ptr)
    };
}
