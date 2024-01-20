#[macro_export]
macro_rules! u32_id {
    ($marker: ty; $id: expr) => {
        $crate::U32Id::<$marker>::from_u32($id)
    };
    ($id: expr) => {
        $crate::U32Id::<_>::from_u32($id)
    };
}
