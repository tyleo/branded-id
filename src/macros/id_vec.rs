/// Builds an [`IdVec`](crate::IdVec), mirroring the `vec!` macro. Takes an optional
/// `Brand` and element type, then either `elem; count` or a comma-separated
/// list of elements.
///
/// # Examples
/// ```rust
/// use branded_id::{id_vec, IdVec};
/// struct Row;
/// let v: IdVec<Row, i32> = id_vec![Row; 1, 2, 3];
/// assert_eq!(v.len(), 3);
/// ```
#[macro_export]
macro_rules! id_vec {
    ($brand:ty; $value:ty) => (
        $crate::IdVec::<$brand, $value>::from_vec(vec![])
    );
    ($brand:ty; $value:ty; $elem:expr; $n:expr) => (
        $crate::IdVec::<$brand, $value>::from_vec(vec![$elem; $n])
    );
    ($brand:ty; $value:ty; $($x:expr),+ $(,)?) => (
        $crate::IdVec::<$brand, $value>::from_vec(vec![$($x),+])
    );
    ($brand:ty) => (
        $crate::IdVec::<$brand, _>::from_vec(vec![])
    );
    ($brand:ty; $elem:expr; $n:expr) => (
        $crate::IdVec::<$brand, _>::from_vec(vec![$elem; $n])
    );
    ($brand:ty; $($x:expr),+ $(,)?) => (
        $crate::IdVec::<$brand, _>::from_vec(vec![$($x),+])
    );
    () => (
        $crate::IdVec::<_, _>::from_vec(vec![])
    );
    ($elem:expr; $n:expr) => (
        $crate::IdVec::<_, _>::from_vec(vec![$elem; $n])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::IdVec::<_, _>::from_vec(vec![$($x),+])
    );
}
