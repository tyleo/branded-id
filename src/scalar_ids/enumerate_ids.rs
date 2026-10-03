use crate::{Id, internal::checked_id};
use std::{fmt, iter::FusedIterator, marker::PhantomData};

/// Pairs each item of an iterator with an id counting from zero.
/// [`IteratorExt::enumerate_ids`](crate::ext::IteratorExt::enumerate_ids) builds
/// one.
pub struct EnumerateIds<TIter, TId> {
    phantom: PhantomData<TId>,
    iter: TIter,
    count: usize,
}

impl<TIter, TId> EnumerateIds<TIter, TId> {
    pub(crate) fn new(iter: TIter) -> Self {
        Self {
            phantom: PhantomData,
            iter,
            count: 0,
        }
    }
}

impl<TIter: Clone, TId> Clone for EnumerateIds<TIter, TId> {
    fn clone(&self) -> Self {
        Self {
            phantom: PhantomData,
            iter: self.iter.clone(),
            count: self.count,
        }
    }
}

impl<TIter: fmt::Debug, TId> fmt::Debug for EnumerateIds<TIter, TId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EnumerateIds")
            .field("iter", &self.iter)
            .field("count", &self.count)
            .finish()
    }
}

impl<TIter: Iterator, TId: Id> Iterator for EnumerateIds<TIter, TId> {
    type Item = (TId, TIter::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;

        let id = id_at(self.count);

        self.count += 1;

        Some((id, item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<TIter: DoubleEndedIterator + ExactSizeIterator, TId: Id> DoubleEndedIterator
    for EnumerateIds<TIter, TId>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.iter.next_back()?;

        Some((id_at(self.count + self.iter.len()), item))
    }
}

impl<TIter: ExactSizeIterator, TId: Id> ExactSizeIterator for EnumerateIds<TIter, TId> {}

impl<TIter: FusedIterator, TId: Id> FusedIterator for EnumerateIds<TIter, TId> {}

fn id_at<TId: Id>(position: usize) -> TId {
    checked_id(position).expect("an enumerated position fits the id width")
}
