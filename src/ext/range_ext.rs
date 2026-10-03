use crate::{Id, IdRange, internal::Sealed};
use std::ops::Range;

crate::internal::id_slice_range_index! { Range, usize_range }

/// Id-typed iteration for a `Range` of ids.
pub trait RangeExt: Sealed {
    /// The range's id type.
    type Id: Id;

    /// Iterates the ids the way a `Range` of their integers iterates.
    fn into_id_range(self) -> IdRange<Self::Id>;
}

impl<TId: Id> RangeExt for Range<TId> {
    type Id = TId;

    fn into_id_range(self) -> IdRange<TId> {
        IdRange::from(self)
    }
}
