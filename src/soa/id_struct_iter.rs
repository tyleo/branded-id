use crate::Id;
use std::slice;

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
