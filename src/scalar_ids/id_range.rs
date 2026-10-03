use crate::{Id, UsizeId, internal::checked_id};
use std::{fmt, iter::FusedIterator, ops::Range};

/// Iterates ids the way a `Range` of their integers iterates.
/// [`RangeExt::into_id_range`](crate::RangeExt::into_id_range) and
/// [`from_len`](Self::from_len) build one.
pub struct IdRange<TId: Id> {
    range: TId::ReprRange,
}

impl<TId: Id> IdRange<TId> {
    /// The first `len` ids from zero.
    ///
    /// # Panics
    /// Panics if `len` does not fit the id width.
    pub fn from_len(len: usize) -> Self {
        let start = TId::from_usize_id(UsizeId::from_usize(0));

        let end = checked_id(len).expect("an id range's end fits its id width");

        Self::from(start..end)
    }
}

impl<TId: Id> Clone for IdRange<TId> {
    fn clone(&self) -> Self {
        Self {
            range: self.range.clone(),
        }
    }
}

impl<TId: Id> fmt::Debug for IdRange<TId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IdRange").field(&self.range).finish()
    }
}

impl<TId: Id> From<Range<TId>> for IdRange<TId> {
    fn from(range: Range<TId>) -> Self {
        Self {
            range: TId::repr_range(range),
        }
    }
}

impl<TId: Id> Iterator for IdRange<TId> {
    type Item = TId;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(TId::from_repr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<TId: Id> DoubleEndedIterator for IdRange<TId> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(TId::from_repr)
    }
}

impl<TId: Id> ExactSizeIterator for IdRange<TId> where TId::ReprRange: ExactSizeIterator {}

impl<TId: Id> FusedIterator for IdRange<TId> {}
