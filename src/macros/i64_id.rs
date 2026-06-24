/// Builds an [`I64Id`](crate::I64Id). Forms: `i64_id!(value)` (brand inferred) and
/// `i64_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{i64_id, I64Id};
/// struct BRow;
/// let id: I64Id<BRow> = i64_id!(BRow; 3);
/// assert_eq!(id.to_i64(), 3);
/// ```
#[macro_export]
macro_rules! i64_id {
    ($id:expr) => {
        $crate::I64Id::<_>::from_i64($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::I64Id::<$brand>::from_i64($id)
    };
}
