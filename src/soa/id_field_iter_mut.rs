use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::Id;

use super::IdStructIter;

/// Iterates the values of an [`IdField`](super::IdField) for the ids
/// retained by an [`IdStruct`](super::IdStruct), yielding mutable
/// references. Created by [`IdField::iter_mut`](super::IdField::iter_mut).
pub struct IdFieldIterMut<'a, TId: Id, TValue> {
    items: *mut MaybeUninit<TValue>,

    len: usize,

    ids: IdStructIter<'a, TId>,

    marker: PhantomData<&'a mut [MaybeUninit<TValue>]>,
}

impl<'a, TId: Id, TValue> IdFieldIterMut<'a, TId, TValue> {
    pub(super) fn new(
        items: *mut MaybeUninit<TValue>,
        len: usize,
        ids: IdStructIter<'a, TId>,
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

impl<'a, TId: Id, TValue> Iterator for IdFieldIterMut<'a, TId, TValue> {
    type Item = &'a mut TValue;

    fn next(&mut self) -> Option<&'a mut TValue> {
        let id = self.ids.next()?;
        // SAFETY: `ids` yields each id at most once, so every returned
        // reference points at a distinct slot; by the IdField::iter_mut
        // contract each retained id has an initialized value, and the field
        // stays borrowed for 'a so the pointer stays valid.
        Some(unsafe { self.at(id.to_usize_id().to_usize()) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ids.size_hint()
    }
}

impl<'a, TId: Id, TValue> DoubleEndedIterator for IdFieldIterMut<'a, TId, TValue> {
    fn next_back(&mut self) -> Option<&'a mut TValue> {
        let id = self.ids.next_back()?;
        // SAFETY: see `next`.
        Some(unsafe { self.at(id.to_usize_id().to_usize()) })
    }
}

impl<TId: Id, TValue> ExactSizeIterator for IdFieldIterMut<'_, TId, TValue> {}
