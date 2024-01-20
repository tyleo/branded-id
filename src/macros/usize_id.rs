#[macro_export]
macro_rules! usize_id {
    ($marker: ty; $id: expr) => {
        $crate::UsizeId::<$marker>::from_usize($id)
    };
    ($id: expr) => {
        $crate::UsizeId::<_>::from_usize($id)
    };
}
