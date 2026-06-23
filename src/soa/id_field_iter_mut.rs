use std::marker::PhantomData;
use std::mem::MaybeUninit;

use super::UsizeIdStructIter;

/// Iterates the values of an [`IdField`](super::IdField) for the ids
/// retained by a [`UsizeIdStruct`](super::UsizeIdStruct), yielding mutable
/// references. Created by [`IdField::iter_mut`](super::IdField::iter_mut).
pub struct IdFieldIterMut<'a, TMarker: ?Sized, TValue> {
    items: *mut MaybeUninit<TValue>,
    len: usize,
    ids: UsizeIdStructIter<'a, TMarker>,
    marker: PhantomData<&'a mut [MaybeUninit<TValue>]>,
}

impl<'a, TMarker: ?Sized, TValue> IdFieldIterMut<'a, TMarker, TValue> {
    pub(super) fn new(
        items: *mut MaybeUninit<TValue>,
        len: usize,
        ids: UsizeIdStructIter<'a, TMarker>,
    ) -> Self {
        Self {
            items,
            len,
            ids,
            marker: PhantomData,
        }
    }

    /// # Safety
    /// `index` must be in range and point at an initialized slot, and each
    /// `index` passed across the lifetime of the iterator must be unique so
    /// the returned references never alias.
    unsafe fn at(&mut self, index: usize) -> &'a mut TValue {
        assert!(index < self.len, "id is out of range for this field");
        let slot = unsafe { &mut *self.items.add(index) };
        unsafe { slot.assume_init_mut() }
    }
}

impl<'a, TMarker: ?Sized, TValue> Iterator for IdFieldIterMut<'a, TMarker, TValue> {
    type Item = &'a mut TValue;

    fn next(&mut self) -> Option<&'a mut TValue> {
        let id = self.ids.next()?;
        // SAFETY: `ids` yields each id at most once, so every returned
        // reference points at a distinct slot; by the IdField::iter_mut
        // contract each retained id has an initialized value, and the field
        // stays borrowed for 'a so the pointer stays valid.
        Some(unsafe { self.at(id.to_usize()) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ids.size_hint()
    }
}

impl<'a, TMarker: ?Sized, TValue> DoubleEndedIterator for IdFieldIterMut<'a, TMarker, TValue> {
    fn next_back(&mut self) -> Option<&'a mut TValue> {
        let id = self.ids.next_back()?;
        // SAFETY: see `next`.
        Some(unsafe { self.at(id.to_usize()) })
    }
}

impl<TMarker: ?Sized, TValue> ExactSizeIterator for IdFieldIterMut<'_, TMarker, TValue> {}
