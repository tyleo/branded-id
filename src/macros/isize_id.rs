/// Builds an [`IsizeId`](crate::IsizeId). Forms: `isize_id!(value)` (brand inferred) and
/// `isize_id!(Brand; value)`.
#[macro_export]
macro_rules! isize_id {
    ($id:expr) => {
        $crate::IsizeId::<_>::from_isize($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::IsizeId::<$brand>::from_isize($id)
    };
}
