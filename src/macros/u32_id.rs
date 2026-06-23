/// Builds a [`U32Id`](crate::U32Id). Forms: `u32_id!(value)` (marker inferred) and
/// `u32_id!(Marker; value)`.
#[macro_export]
macro_rules! u32_id {
    ($id: expr) => {
        $crate::U32Id::<_>::from_u32($id)
    };
    ($marker: ty; $id: expr) => {
        $crate::U32Id::<$marker>::from_u32($id)
    };
}
