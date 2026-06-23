/// Builds an [`IdSlice`](crate::IdSlice) borrowing a temporary array. Takes an optional
/// `Brand` and element type, then either `elem; count` or a comma-separated
/// list of elements.
#[macro_export]
macro_rules! id_slice {
    ($brand: ty; $value: ty) => (
        $crate::IdSlice::<$brand, $value>::from_slice(&[])
    );
    ($brand: ty; $value: ty; $elem: expr; $n: expr) => (
        $crate::IdSlice::<$brand, $value>::from_slice(&[$elem; $n])
    );
    ($brand: ty; $value: ty; $($x: expr),+ $(,)?) => (
        $crate::IdSlice::<$brand, $value>::from_slice(&[$($x),+])
    );
    ($brand: ty) => (
        $crate::IdSlice::<$brand, _>::from_slice(&[])
    );
    ($brand: ty; $elem: expr; $n: expr) => (
        $crate::IdSlice::<$brand, _>::from_slice(&[$elem; $n])
    );
    ($brand: ty; $($x: expr),+ $(,)?) => (
        $crate::IdSlice::<$brand, _>::from_slice(&[$($x),+])
    );
    () => (
        $crate::IdSlice::<_, _>::from_slice(&[])
    );
    ($elem: expr; $n: expr) => (
        $crate::IdSlice::<_, _>::from_slice(&[$elem; $n])
    );
    ($($x: expr),+ $(,)?) => (
        $crate::IdSlice::<_, _>::from_slice(&[$($x),+])
    );
}
