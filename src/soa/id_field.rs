use crate::{IdSlice, IdSliceIndex, IdVec, UsizeId};
use std::{
    mem::{transmute, MaybeUninit},
    ops::{Index, IndexMut},
};

pub struct IdField<TMarker: ?Sized, T> {
    items: IdVec<TMarker, MaybeUninit<T>>,
}

impl<TMarker: ?Sized, T> IdField<TMarker, T> {
    pub fn new() -> Self {
        Self {
            items: IdVec::new(),
        }
    }

    /// # Safety
    /// A value must be `retain()`'d at the id for `release()`` to be safe to call.
    pub unsafe fn release_all(mut self, iter: impl IntoIterator<Item = UsizeId<TMarker>>) {
        for id in iter {
            self.release(id)
        }
    }

    pub fn retain(&mut self, id: UsizeId<TMarker>, value: T) {
        ensure_size(&mut self.items, id.to_usize() + 1);
        self.items[id].write(value);
    }

    /// # Safety
    /// A value must be `retain()`'d at the id for `release()`` to be safe to call.
    pub unsafe fn release(&mut self, id: UsizeId<TMarker>) {
        let item = &mut self.items[id];
        MaybeUninit::assume_init_drop(item)
    }
}

fn ensure_size<TMarker: ?Sized, T>(
    items: &mut IdVec<TMarker, MaybeUninit<T>>,
    desired_size: usize,
) {
    while items.len() < desired_size {
        items.push(MaybeUninit::uninit());
    }
}

impl<TMarker: ?Sized, T> Default for IdField<TMarker, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TMarker: ?Sized, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> Index<I>
    for IdField<TMarker, TValue>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        unsafe {
            let items_actual = transmute::<_, &IdVec<TMarker, TValue>>(&self.items);
            items_actual.index(index)
        }
    }
}

impl<TMarker: ?Sized, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> IndexMut<I>
    for IdField<TMarker, TValue>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        unsafe {
            let items_actual = transmute::<_, &mut IdVec<TMarker, TValue>>(&mut self.items);
            items_actual.index_mut(index)
        }
    }
}
