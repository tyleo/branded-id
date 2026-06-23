/// Builds an [`IsizeId`](crate::IsizeId). Forms: `isize_id!(value)` (brand inferred) and
/// `isize_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{isize_id, IsizeId};
/// struct Row;
/// let id: IsizeId<Row> = isize_id!(Row; 3);
/// assert_eq!(id.to_isize(), 3);
/// ```
#[macro_export]
macro_rules! isize_id {
    ($id:expr) => {
        $crate::IsizeId::<_>::from_isize($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::IsizeId::<$brand>::from_isize($id)
    };
}
