/// Builds a [`U128Id`](crate::U128Id). Forms: `u128_id!(value)` (brand inferred) and
/// `u128_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{u128_id, U128Id};
/// struct BRow;
/// let id: U128Id<BRow> = u128_id!(BRow; 3);
/// assert_eq!(id.to_u128(), 3);
/// ```
#[macro_export]
macro_rules! u128_id {
    ($id:expr) => {
        $crate::U128Id::<_>::from_u128($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::U128Id::<$brand>::from_u128($id)
    };
}
