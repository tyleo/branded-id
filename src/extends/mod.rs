//! Marker-subtyping conversions between id domains.
//!
//! When one marker [`Extends`] another, the id types and containers can be
//! cast between the two markers via their `upcast_*` and `downcast_*`
//! methods.

/// Declares a marker-subtyping relationship: `impl Extends<Base> for Derived`
/// says that ids and containers marked `Derived` may be converted to and from
/// the `Base` marker through the cast methods on the id types.
pub trait Extends<TMarker: ?Sized> {}

mod i32_id;
mod id_array;
mod id_ptr;
mod id_slice;
mod id_vec;
mod isize_id;
mod mut_id_ptr;
mod u32_id;
mod usize_id;
