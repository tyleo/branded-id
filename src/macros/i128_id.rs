/// Builds an [`I128Id`](crate::I128Id). Forms: `i128_id!(value)` (brand inferred) and
/// `i128_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{i128_id, I128Id};
/// struct BRow;
/// let id: I128Id<BRow> = i128_id!(BRow; 3);
/// assert_eq!(id.to_i128(), 3);
/// ```
#[macro_export]
macro_rules! i128_id {
    ($id:expr) => {
        $crate::I128Id::<_>::from_i128($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::I128Id::<$brand>::from_i128($id)
    };
}
