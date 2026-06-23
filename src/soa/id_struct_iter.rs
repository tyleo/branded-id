use crate::Id;
use std::{fmt, slice};

/// Iterates the ids currently retained by an [`IdStruct`](super::IdStruct),
/// in the order they appear in the packed `live` list.
pub struct IdStructIter<'a, TId> {
    live: slice::Iter<'a, TId>,
}

impl<'a, TId> IdStructIter<'a, TId> {
    pub(super) fn from_live(live: &'a [TId]) -> Self {
        Self { live: live.iter() }
    }
}

impl<TId> Clone for IdStructIter<'_, TId> {
    fn clone(&self) -> Self {
        // `slice::Iter` is `Clone` for any element type, so no `TId` bound is
        // needed; the clone is an independent cursor over the same ids.
        Self {
            live: self.live.clone(),
        }
    }
}

impl<TId: fmt::Debug> fmt::Debug for IdStructIter<'_, TId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IdStructIter")
            .field(&self.live.as_slice())
            .finish()
    }
}

impl<TId: Id> Iterator for IdStructIter<'_, TId> {
    type Item = TId;

    fn next(&mut self) -> Option<Self::Item> {
        self.live.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.live.size_hint()
    }
}

impl<TId: Id> DoubleEndedIterator for IdStructIter<'_, TId> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.live.next_back().copied()
    }
}

impl<TId: Id> ExactSizeIterator for IdStructIter<'_, TId> {}
