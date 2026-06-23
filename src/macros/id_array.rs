/// Builds an [`IdArray`](crate::IdArray), mirroring array literals. Takes an optional
/// `Brand` and element type, then either `elem; count` or a comma-separated
/// list of elements; the length `N` is inferred.
///
/// # Examples
/// ```rust
/// use branded_id::{id_array, usize_id, IdArray, UsizeId};
/// struct BRow;
/// let a: IdArray<BRow, i32, 3> = id_array![BRow; 1, 2, 3];
/// let id: UsizeId<BRow> = usize_id!(BRow; 2);
/// assert_eq!(a[id], 3);
/// ```
#[macro_export]
macro_rules! id_array {
    ($brand:ty; $value:ty) => (
        $crate::IdArray::<$brand, $value, 0>::from_array([])
    );
    ($brand:ty; $value:ty; $elem:expr; $n:expr) => (
        $crate::IdArray::<$brand, $value, $n>::from_array([$elem; $n])
    );
    ($brand:ty; $value:ty; $($x:expr),+ $(,)?) => (
        $crate::IdArray::<$brand, $value, _>::from_array([$($x),+])
    );
    ($brand:ty) => (
        $crate::IdArray::<$brand, _, 0>::from_array([])
    );
    ($brand:ty; $elem:expr; $n:expr) => (
        $crate::IdArray::<$brand, _, $n>::from_array([$elem; $n])
    );
    ($brand:ty; $($x:expr),+ $(,)?) => (
        $crate::IdArray::<$brand, _, _>::from_array([$($x),+])
    );
    () => (
        $crate::IdArray::<_, _, 0>::from_array([])
    );
    ($elem:expr; $n:expr) => (
        $crate::IdArray::<_, _, $n>::from_array([$elem; $n])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::IdArray::<_, _, _>::from_array([$($x),+])
    );
}
