/// Builds a [`U32Id`](crate::U32Id). Forms: `u32_id!(value)` (brand inferred) and
/// `u32_id!(Brand; value)`.
#[macro_export]
macro_rules! u32_id {
    ($id: expr) => {
        $crate::U32Id::<_>::from_u32($id)
    };
    ($brand: ty; $id: expr) => {
        $crate::U32Id::<$brand>::from_u32($id)
    };
}
