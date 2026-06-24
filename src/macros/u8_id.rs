/// Builds a [`U8Id`](crate::U8Id). Forms: `u8_id!(value)` (brand inferred) and
/// `u8_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{u8_id, U8Id};
/// struct BRow;
/// let id: U8Id<BRow> = u8_id!(BRow; 3);
/// assert_eq!(id.to_u8(), 3);
/// ```
#[macro_export]
macro_rules! u8_id {
    ($id:expr) => {
        $crate::U8Id::<_>::from_u8($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::U8Id::<$brand>::from_u8($id)
    };
}
