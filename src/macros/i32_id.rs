/// Builds an [`I32Id`](crate::I32Id). Forms: `i32_id!(value)` (brand inferred) and
/// `i32_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{i32_id, I32Id};
/// struct BRow;
/// let id: I32Id<BRow> = i32_id!(BRow; 3);
/// assert_eq!(id.to_i32(), 3);
/// ```
#[macro_export]
macro_rules! i32_id {
    ($id:expr) => {
        $crate::I32Id::<_>::from_i32($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::I32Id::<$brand>::from_i32($id)
    };
}
