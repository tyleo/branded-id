/// Builds a [`U64Id`](crate::U64Id). Forms: `u64_id!(value)` (brand inferred) and
/// `u64_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{u64_id, U64Id};
/// struct BRow;
/// let id: U64Id<BRow> = u64_id!(BRow; 3);
/// assert_eq!(id.to_u64(), 3);
/// ```
#[macro_export]
macro_rules! u64_id {
    ($id:expr) => {
        $crate::U64Id::<_>::from_u64($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::U64Id::<$brand>::from_u64($id)
    };
}
