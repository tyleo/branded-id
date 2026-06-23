/// Builds a [`UsizeId`](crate::UsizeId). Forms: `usize_id!(value)` (brand inferred) and
/// `usize_id!(Brand; value)`.
#[macro_export]
macro_rules! usize_id {
    ($id:expr) => {
        $crate::UsizeId::<_>::from_usize($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::UsizeId::<$brand>::from_usize($id)
    };
}
