/// Builds an [`IdArray`](crate::IdArray), mirroring array literals. Takes an optional
/// `Marker` and element type, then either `elem; count` or a comma-separated
/// list of elements; the length `N` is inferred.
#[macro_export]
macro_rules! id_array {
    ($marker: ty; $value: ty) => (
        $crate::IdArray::<$marker, $value, 0>::from_array([])
    );
    ($marker: ty; $value: ty; $elem: expr; $n: expr) => (
        $crate::IdArray::<$marker, $value, $n>::from_array([$elem; $n])
    );
    ($marker: ty; $value: ty; $($x: expr),+ $(,)?) => (
        $crate::IdArray::<$marker, $value, _>::from_array([$($x),+])
    );
    ($marker: ty) => (
        $crate::IdArray::<$marker, _, 0>::from_array([])
    );
    ($marker: ty; $elem: expr; $n: expr) => (
        $crate::IdArray::<$marker, _, $n>::from_array([$elem; $n])
    );
    ($marker: ty; $($x: expr),+ $(,)?) => (
        $crate::IdArray::<$marker, _, _>::from_array([$($x),+])
    );
    () => (
        $crate::IdArray::<_, _, 0>::from_array([])
    );
    ($elem: expr; $n: expr) => (
        $crate::IdArray::<_, _, $n>::from_array([$elem; $n])
    );
    ($($x: expr),+ $(,)?) => (
        $crate::IdArray::<_, _, _>::from_array([$($x),+])
    );
}
