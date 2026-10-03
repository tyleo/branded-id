//! Brand-typed UUID ids.
//!
//! Like the string ids, and unlike the integer ids, these are opaque branded
//! keys rather than indices: a [`Uuid`](uuid::Uuid) newtype carrying a `TBrand`
//! so ids built for different domains are distinct types even though they share
//! a `Uuid` representation. Convert to and from a raw `Uuid` with
//! [`from_uuid`](UuidId::from_uuid)/[`to_uuid`](UuidId::to_uuid). The
//! [`uuid_id!`](crate::uuid_id!) macro builds them.
//!
//! This module is behind the optional, non-default `uuid` feature.

mod u128_id;
mod uuid_id;

pub use uuid_id::*;
