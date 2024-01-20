#[macro_export]
macro_rules! isize_id {
    ($marker: ty; $id: expr) => {
        $crate::ISizeId::<$marker>::from_isize($id)
    };
    ($id: expr) => {
        $crate::ISizeId::<_>::from_isize($id)
    };
}
