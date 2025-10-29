// This trait being unreachable from outside this crate prevents outside
// implementations of our extension traits. This allows adding more trait
// methods in the future.
pub trait Sealed {}

impl<T: ?Sized> Sealed for T {}
