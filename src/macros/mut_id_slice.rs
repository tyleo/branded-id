#[macro_export]
macro_rules! mut_id_slice {
    ($marker: ty; $value: ty) => (
        $crate::IdSlice::<$marker, $value>::from_mut_slice(&mut [])
    );
    ($marker: ty; $value: ty; $elem: expr; $n: expr) => (
        $crate::IdSlice::<$marker, $value>::from_mut_slice(&mut [$elem; $n])
    );
    ($marker: ty; $value: ty; $($x: expr),+ $(,)?) => (
        $crate::IdSlice::<$marker, $value>::from_mut_slice(&mut [$($x),+])
    );
    ($marker: ty) => (
        $crate::IdSlice::<$marker, _>::from_mut_slice(&mut [])
    );
    ($marker: ty; $elem: expr; $n: expr) => (
        $crate::IdSlice::<$marker, _>::from_mut_slice(&mut [$elem; $n])
    );
    ($marker: ty; $($x: expr),+ $(,)?) => (
        $crate::IdSlice::<$marker, _>::from_mut_slice(&mut [$($x),+])
    );
    () => (
        $crate::IdSlice::<_, _>::from_mut_slice(&mut [])
    );
    ($elem: expr; $n: expr) => (
        $crate::IdSlice::<_, _>::from_mut_slice(&mut [$elem; $n])
    );
    ($($x: expr),+ $(,)?) => (
        $crate::IdSlice::<_, _>::from_mut_slice(&mut [$($x),+])
    );
}
