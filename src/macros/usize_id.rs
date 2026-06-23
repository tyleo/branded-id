/// Builds a [`UsizeId`](crate::UsizeId). Forms: `usize_id!(value)` (brand inferred) and
/// `usize_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{usize_id, UsizeId};
/// struct BRow;
/// let id: UsizeId<BRow> = usize_id!(BRow; 3);
/// assert_eq!(id.to_usize(), 3);
/// ```
#[macro_export]
macro_rules! usize_id {
    ($id:expr) => {
        $crate::UsizeId::<_>::from_usize($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::UsizeId::<$brand>::from_usize($id)
    };
}
