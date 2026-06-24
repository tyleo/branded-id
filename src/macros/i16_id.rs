/// Builds an [`I16Id`](crate::I16Id). Forms: `i16_id!(value)` (brand inferred) and
/// `i16_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{i16_id, I16Id};
/// struct BRow;
/// let id: I16Id<BRow> = i16_id!(BRow; 3);
/// assert_eq!(id.to_i16(), 3);
/// ```
#[macro_export]
macro_rules! i16_id {
    ($id:expr) => {
        $crate::I16Id::<_>::from_i16($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::I16Id::<$brand>::from_i16($id)
    };
}
