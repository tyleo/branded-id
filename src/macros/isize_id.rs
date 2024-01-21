#[macro_export]
macro_rules! isize_id {
    ($id: expr) => {
        $crate::IsizeId::<_>::from_isize($id)
    };
    ($marker: ty; $id: expr) => {
        $crate::IsizeId::<$marker>::from_isize($id)
    };
}
