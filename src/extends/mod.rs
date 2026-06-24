//! Brand-subtyping conversions between id domains.
//!
//! When one brand [`Extends`] another, the id types and containers can be
//! cast between the two brands via their `upcast_*` and `downcast_*`
//! methods.

/// Declares a brand-subtyping relationship: `impl Extends<Base> for Derived`
/// says that ids and containers branded `Derived` may be converted to and from
/// the `Base` brand through the cast methods on the id types.
pub trait Extends<TBrand: ?Sized> {}

mod c_str_id;
mod i128_id;
mod i16_id;
mod i32_id;
mod i64_id;
mod i8_id;
mod id_array;
mod id_ptr;
mod id_slice;
mod id_vec;
mod isize_id;
mod mut_id_ptr;
mod os_str_id;
mod path_id;
mod str_id;
mod u128_id;
mod u16_id;
mod u32_id;
mod u64_id;
mod u8_id;
mod usize_id;
