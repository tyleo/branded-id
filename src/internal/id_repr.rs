use std::{
    fmt::Debug,
    iter::FusedIterator,
    ops::{Range, RangeInclusive},
};

/// The primitive behind a scalar id, so generic id ranges iterate the way the
/// primitive's ranges iterate.
pub trait IdRepr: Sized {
    /// The primitive integer the id wraps.
    type Repr;

    /// The primitive's `Range`.
    type ReprRange: Clone + Debug + DoubleEndedIterator<Item = Self::Repr> + FusedIterator;

    /// The primitive's `RangeInclusive`.
    type ReprRangeInclusive: Clone + Debug + DoubleEndedIterator<Item = Self::Repr> + FusedIterator;

    fn from_repr(repr: Self::Repr) -> Self;

    fn repr_range(range: Range<Self>) -> Self::ReprRange;

    fn repr_range_inclusive(range: RangeInclusive<Self>) -> Self::ReprRangeInclusive;
}
