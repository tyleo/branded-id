use crate::{U32Id, soa::IdStruct};

/// An [`IdStruct`] keyed by [`U32Id`], the common 32-bit specialization.
pub type U32IdStruct<TMarker> = IdStruct<U32Id<TMarker>>;
