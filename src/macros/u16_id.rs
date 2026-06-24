/// Builds a [`U16Id`](crate::U16Id). Forms: `u16_id!(value)` (brand inferred) and
/// `u16_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{u16_id, U16Id};
/// struct BRow;
/// let id: U16Id<BRow> = u16_id!(BRow; 3);
/// assert_eq!(id.to_u16(), 3);
/// ```
#[macro_export]
macro_rules! u16_id {
    ($id:expr) => {
        $crate::U16Id::<_>::from_u16($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::U16Id::<$brand>::from_u16($id)
    };
}
