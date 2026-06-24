//! Brand-typed integer ids.
//!
//! A brand-typed id for each primitive integer width, every one a
//! `#[repr(transparent)]` newtype that carries a `TBrand` so ids built for
//! different domains are distinct types even though they share an integer
//! representation. [`UsizeId`] is the canonical width that indexes the
//! containers directly; the other widths convert through it with the `to_*_id`
//! methods. The [`Id`] and [`Scalar`] traits let generic code abstract over the
//! width an id uses.

mod i128_id;
mod i16_id;
mod i32_id;
mod i64_id;
mod i8_id;
mod id;
mod id_conversions;
mod isize_id;
mod scalar;
mod scalar_id_conversions;
mod scalar_id_impl;
mod u128_id;
mod u16_id;
mod u32_id;
mod u64_id;
mod u8_id;
mod usize_id;

pub(crate) use scalar_id_conversions::scalar_id_conversions;
pub(crate) use scalar_id_impl::scalar_id_impl;

pub use i8_id::*;
pub use i16_id::*;
pub use i32_id::*;
pub use i64_id::*;
pub use i128_id::*;
pub use id::*;
pub use isize_id::*;
pub use scalar::*;
pub use u8_id::*;
pub use u16_id::*;
pub use u32_id::*;
pub use u64_id::*;
pub use u128_id::*;
pub use usize_id::*;
