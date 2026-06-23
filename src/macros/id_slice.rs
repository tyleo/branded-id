/// Builds an [`IdSlice`](crate::IdSlice) borrowing a temporary array. Takes an optional
/// `Brand` and element type, then either `elem; count` or a comma-separated
/// list of elements.
///
/// # Examples
/// ```rust
/// use branded_id::{id_slice, IdSlice};
/// struct BRow;
/// let s: &IdSlice<BRow, i32> = id_slice![BRow; 1, 2, 3];
/// assert_eq!(s.len(), 3);
/// ```
#[macro_export]
macro_rules! id_slice {
    ($brand:ty; $value:ty) => (
        $crate::IdSlice::<$brand, $value>::from_slice(&[])
    );
    ($brand:ty; $value:ty; $elem:expr; $n:expr) => (
        $crate::IdSlice::<$brand, $value>::from_slice(&[$elem; $n])
    );
    ($brand:ty; $value:ty; $($x:expr),+ $(,)?) => (
        $crate::IdSlice::<$brand, $value>::from_slice(&[$($x),+])
    );
    ($brand:ty) => (
        $crate::IdSlice::<$brand, _>::from_slice(&[])
    );
    ($brand:ty; $elem:expr; $n:expr) => (
        $crate::IdSlice::<$brand, _>::from_slice(&[$elem; $n])
    );
    ($brand:ty; $($x:expr),+ $(,)?) => (
        $crate::IdSlice::<$brand, _>::from_slice(&[$($x),+])
    );
    () => (
        $crate::IdSlice::<_, _>::from_slice(&[])
    );
    ($elem:expr; $n:expr) => (
        $crate::IdSlice::<_, _>::from_slice(&[$elem; $n])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::IdSlice::<_, _>::from_slice(&[$($x),+])
    );
}
