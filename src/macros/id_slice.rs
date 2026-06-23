/// Builds an [`IdSlice`](crate::IdSlice) borrowing a temporary array. Takes an optional
/// `Marker` and element type, then either `elem; count` or a comma-separated
/// list of elements.
#[macro_export]
macro_rules! id_slice {
    ($marker: ty; $value: ty) => (
        $crate::IdSlice::<$marker, $value>::from_slice(&[])
    );
    ($marker: ty; $value: ty; $elem: expr; $n: expr) => (
        $crate::IdSlice::<$marker, $value>::from_slice(&[$elem; $n])
    );
    ($marker: ty; $value: ty; $($x: expr),+ $(,)?) => (
        $crate::IdSlice::<$marker, $value>::from_slice(&[$($x),+])
    );
    ($marker: ty) => (
        $crate::IdSlice::<$marker, _>::from_slice(&[])
    );
    ($marker: ty; $elem: expr; $n: expr) => (
        $crate::IdSlice::<$marker, _>::from_slice(&[$elem; $n])
    );
    ($marker: ty; $($x: expr),+ $(,)?) => (
        $crate::IdSlice::<$marker, _>::from_slice(&[$($x),+])
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
