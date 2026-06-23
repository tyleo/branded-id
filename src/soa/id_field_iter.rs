use std::mem::MaybeUninit;

use super::UsizeIdStructIter;

/// Iterates the values of an [`IdField`](super::IdField) for the ids
/// retained by a [`UsizeIdStruct`](super::UsizeIdStruct), yielding shared
/// references. Created by [`IdField::iter`](super::IdField::iter).
pub struct IdFieldIter<'a, TMarker: ?Sized, TValue> {
    items: &'a [MaybeUninit<TValue>],
    ids: UsizeIdStructIter<'a, TMarker>,
}

impl<'a, TMarker: ?Sized, TValue> IdFieldIter<'a, TMarker, TValue> {
    pub(super) fn new(
        items: &'a [MaybeUninit<TValue>],
        ids: UsizeIdStructIter<'a, TMarker>,
    ) -> Self {
        Self { items, ids }
    }
}

impl<'a, TMarker: ?Sized, TValue> Iterator for IdFieldIter<'a, TMarker, TValue> {
    type Item = &'a TValue;

    fn next(&mut self) -> Option<&'a TValue> {
        let id = self.ids.next()?;
        let items = self.items;
        // SAFETY: by the IdField::iter contract, every id retained by the
        // pool has an initialized value in this field.
        Some(unsafe { items[id.to_usize()].assume_init_ref() })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ids.size_hint()
    }
}

impl<'a, TMarker: ?Sized, TValue> DoubleEndedIterator for IdFieldIter<'a, TMarker, TValue> {
    fn next_back(&mut self) -> Option<&'a TValue> {
        let id = self.ids.next_back()?;
        let items = self.items;
        // SAFETY: see `next`.
        Some(unsafe { items[id.to_usize()].assume_init_ref() })
    }
}

impl<TMarker: ?Sized, TValue> ExactSizeIterator for IdFieldIter<'_, TMarker, TValue> {}
