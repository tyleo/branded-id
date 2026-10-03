use crate::{Id, UsizeId, internal::checked_id};
use std::{fmt, iter::FusedIterator, marker::PhantomData, ops::Range};

/// Iterates the ids from zero up to an end id. [`from_len`](Self::from_len),
/// [`UsizeId::range_from_zero`] and the matching method on every other id width
/// build one.
pub struct IdRange<TId> {
    phantom: PhantomData<TId>,
    indices: Range<usize>,
}

impl<TId> IdRange<TId> {
    pub(crate) fn from_usize_range(indices: Range<usize>) -> Self {
        Self {
            phantom: PhantomData,
            indices,
        }
    }
}

impl<TId: Id> IdRange<TId> {
    /// The first `len` ids from zero.
    ///
    /// # Panics
    /// Panics if the last id does not fit the id width.
    pub fn from_len(len: usize) -> Self {
        if let Some(last) = len.checked_sub(1) {
            assert!(
                checked_id::<TId>(last).is_some(),
                "an id range's last id fits its id width"
            );
        }

        Self::from_usize_range(0..len)
    }
}

impl<TId> Clone for IdRange<TId> {
    fn clone(&self) -> Self {
        Self::from_usize_range(self.indices.clone())
    }
}

impl<TId> fmt::Debug for IdRange<TId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IdRange").field(&self.indices).finish()
    }
}

impl<TId: Id> Iterator for IdRange<TId> {
    type Item = TId;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next()?;
        Some(TId::from_usize_id(UsizeId::from_usize(index)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl<TId: Id> DoubleEndedIterator for IdRange<TId> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.indices.next_back()?;
        Some(TId::from_usize_id(UsizeId::from_usize(index)))
    }
}

impl<TId: Id> ExactSizeIterator for IdRange<TId> {}

impl<TId: Id> FusedIterator for IdRange<TId> {}
