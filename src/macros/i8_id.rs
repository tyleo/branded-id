/// Builds an [`I8Id`](crate::I8Id). Forms: `i8_id!(value)` (brand inferred) and
/// `i8_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{i8_id, I8Id};
/// struct BRow;
/// let id: I8Id<BRow> = i8_id!(BRow; 3);
/// assert_eq!(id.to_i8(), 3);
/// ```
#[macro_export]
macro_rules! i8_id {
    ($id:expr) => {
        $crate::I8Id::<_>::from_i8($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::I8Id::<$brand>::from_i8($id)
    };
}
