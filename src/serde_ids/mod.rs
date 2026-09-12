//! `Serialize` and `Deserialize` for the ids, each as its bare
//! representation: a scalar id as its integer and a `UuidId` as its `Uuid`.
//! The brand is a compile-time fact and never reaches the wire.

mod i128_id;
mod i16_id;
mod i32_id;
mod i64_id;
mod i8_id;
mod isize_id;
mod scalar_id_serde;
mod u128_id;
mod u16_id;
mod u32_id;
mod u64_id;
mod u8_id;
mod usize_id;

#[cfg(feature = "uuid")]
mod uuid_id;

pub(crate) use scalar_id_serde::scalar_id_serde;
