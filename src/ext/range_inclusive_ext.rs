use crate::{Id, IdRangeInclusive, internal::Sealed};
use std::ops::RangeInclusive;

crate::internal::id_slice_range_index! { RangeInclusive, usize_range_inclusive }

/// Id-typed iteration for a `RangeInclusive` of ids.
pub trait RangeInclusiveExt: Sealed {
    /// The range's id type.
    type Id: Id;

    /// Iterates the ids the way a `RangeInclusive` of their integers iterates.
    fn into_id_range(self) -> IdRangeInclusive<Self::Id>;
}

impl<TId: Id> RangeInclusiveExt for RangeInclusive<TId> {
    type Id = TId;

    fn into_id_range(self) -> IdRangeInclusive<TId> {
        IdRangeInclusive::from(self)
    }
}
