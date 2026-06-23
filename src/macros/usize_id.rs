/// Builds a [`UsizeId`](crate::UsizeId). Forms: `usize_id!(value)` (marker inferred) and
/// `usize_id!(Marker; value)`.
#[macro_export]
macro_rules! usize_id {
    ($id: expr) => {
        $crate::UsizeId::<_>::from_usize($id)
    };
    ($marker: ty; $id: expr) => {
        $crate::UsizeId::<$marker>::from_usize($id)
    };
}
