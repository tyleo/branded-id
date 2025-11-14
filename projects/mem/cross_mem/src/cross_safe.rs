/// Marker trait for types that are safe to transfer across module boundaries.
/// # Safety
/// Implementing this trait for a type indicates that it is safe to transfer
/// across module boundaries. The type must not contain any references, pointers,
/// or other non-Copy data that could lead to undefined behavior when accessed
/// in a different module.
pub unsafe trait CrossSafe: Copy + Sized + 'static {}

unsafe impl CrossSafe for u8 {}
unsafe impl CrossSafe for u16 {}
unsafe impl CrossSafe for u32 {}
unsafe impl CrossSafe for u64 {}
unsafe impl CrossSafe for usize {}
unsafe impl CrossSafe for i8 {}
unsafe impl CrossSafe for i16 {}
unsafe impl CrossSafe for i32 {}
unsafe impl CrossSafe for i64 {}
unsafe impl CrossSafe for isize {}
unsafe impl CrossSafe for f32 {}
unsafe impl CrossSafe for f64 {}
unsafe impl<T: 'static> CrossSafe for *const T {}
unsafe impl<T: 'static> CrossSafe for *mut T {}
