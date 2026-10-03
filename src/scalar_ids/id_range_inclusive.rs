use crate::Id;
use std::{fmt, iter::FusedIterator, ops::RangeInclusive};

/// Iterates ids the way a `RangeInclusive` of their integers iterates.
/// [`RangeInclusiveExt::into_id_range`](crate::ext::RangeInclusiveExt::into_id_range)
/// builds one.
pub struct IdRangeInclusive<TId: Id> {
    range: TId::ReprRangeInclusive,
}

impl<TId: Id> Clone for IdRangeInclusive<TId> {
    fn clone(&self) -> Self {
        Self {
            range: self.range.clone(),
        }
    }
}

impl<TId: Id> fmt::Debug for IdRangeInclusive<TId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IdRangeInclusive")
            .field(&self.range)
            .finish()
    }
}

impl<TId: Id> From<RangeInclusive<TId>> for IdRangeInclusive<TId> {
    fn from(range: RangeInclusive<TId>) -> Self {
        Self {
            range: TId::repr_range_inclusive(range),
        }
    }
}

impl<TId: Id> Iterator for IdRangeInclusive<TId> {
    type Item = TId;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(TId::from_repr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<TId: Id> DoubleEndedIterator for IdRangeInclusive<TId> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(TId::from_repr)
    }
}

impl<TId: Id> ExactSizeIterator for IdRangeInclusive<TId> where
    TId::ReprRangeInclusive: ExactSizeIterator
{
}

impl<TId: Id> FusedIterator for IdRangeInclusive<TId> {}
