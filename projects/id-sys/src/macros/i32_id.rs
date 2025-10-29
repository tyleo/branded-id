#[macro_export]
macro_rules! i32_id {
    ($id: expr) => {
        $crate::I32Id::<_>::from_i32($id)
    };
    ($marker: ty; $id: expr) => {
        $crate::I32Id::<$marker>::from_i32($id)
    };
}
