use std::mem::MaybeUninit;

use crate::Id;

use super::IdStructIter;

/// Iterates the values of an [`IdField`](super::IdField) for the ids
/// retained by an [`IdStruct`](super::IdStruct), yielding shared
/// references. Created by [`IdField::iter`](super::IdField::iter).
pub struct IdFieldIter<'a, TId: Id, TValue> {
    items: &'a [MaybeUninit<TValue>],

    ids: IdStructIter<'a, TId>,
}

impl<'a, TId: Id, TValue> IdFieldIter<'a, TId, TValue> {
    pub(super) fn new(items: &'a [MaybeUninit<TValue>], ids: IdStructIter<'a, TId>) -> Self {
        Self { items, ids }
    }
}

impl<'a, TId: Id, TValue> Clone for IdFieldIter<'a, TId, TValue> {
    fn clone(&self) -> Self {
        // Only the shared slice reference (`Copy`) and the id cursor are
        // duplicated; the values are never touched, so no `TValue` bound is
        // needed and the clone hands out the same shared references.
        Self {
            items: self.items,
            ids: self.ids.clone(),
        }
    }
}

impl<'a, TId: Id, TValue> Iterator for IdFieldIter<'a, TId, TValue> {
    type Item = &'a TValue;

    fn next(&mut self) -> Option<&'a TValue> {
        let id = self.ids.next()?;
        let items = self.items;
        // SAFETY: by the IdField::iter contract, every id retained by the
        // pool has an initialized value in this field.
        Some(unsafe { items[id.to_usize_id().to_usize()].assume_init_ref() })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ids.size_hint()
    }
}

impl<'a, TId: Id, TValue> DoubleEndedIterator for IdFieldIter<'a, TId, TValue> {
    fn next_back(&mut self) -> Option<&'a TValue> {
        let id = self.ids.next_back()?;
        let items = self.items;
        // SAFETY: see `next`.
        Some(unsafe { items[id.to_usize_id().to_usize()].assume_init_ref() })
    }
}

impl<TId: Id, TValue> ExactSizeIterator for IdFieldIter<'_, TId, TValue> {}
