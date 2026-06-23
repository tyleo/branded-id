use crate::soa::IdStruct;

/// An [`IdStruct`] keyed by `u32`, the common 32-bit specialization. Since
/// `u32` is the default integer width, this is the same type as writing the
/// brand alone, `IdStruct<TBrand>`.
pub type U32IdStruct<TBrand> = IdStruct<TBrand, u32>;
