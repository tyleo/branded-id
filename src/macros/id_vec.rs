/// Builds an [`IdVec`](crate::IdVec), mirroring the `vec!` macro. Takes an optional
/// `Marker` and element type, then either `elem; count` or a comma-separated
/// list of elements.
#[macro_export]
macro_rules! id_vec {
    ($marker: ty; $value: ty) => (
        $crate::IdVec::<$marker, $value>::from_vec(vec![])
    );
    ($marker: ty; $value: ty; $elem: expr; $n: expr) => (
        $crate::IdVec::<$marker, $value>::from_vec(vec![$elem; $n])
    );
    ($marker: ty; $value: ty; $($x: expr),+ $(,)?) => (
        $crate::IdVec::<$marker, $value>::from_vec(vec![$($x),+])
    );
    ($marker: ty) => (
        $crate::IdVec::<$marker, _>::from_vec(vec![])
    );
    ($marker: ty; $elem: expr; $n: expr) => (
        $crate::IdVec::<$marker, _>::from_vec(vec![$elem; $n])
    );
    ($marker: ty; $($x: expr),+ $(,)?) => (
        $crate::IdVec::<$marker, _>::from_vec(vec![$($x),+])
    );
    () => (
        $crate::IdVec::<_, _>::from_vec(vec![])
    );
    ($elem: expr; $n: expr) => (
        $crate::IdVec::<_, _>::from_vec(vec![$elem; $n])
    );
    ($($x: expr),+ $(,)?) => (
        $crate::IdVec::<_, _>::from_vec(vec![$($x),+])
    );
}
