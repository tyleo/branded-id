use crate::{IdSlice, IdSliceIndex, IdVec, UsizeId};
use std::{
    mem::{transmute, MaybeUninit},
    ops::{Index, IndexMut},
};

pub struct IdField<TMarker: ?Sized, TValue> {
    items: IdVec<TMarker, MaybeUninit<TValue>>,
}

impl<TMarker: ?Sized, TValue> IdField<TMarker, TValue> {
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

    pub fn retain(&mut self, id: UsizeId<TMarker>, value: TValue) {
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

fn ensure_size<TMarker: ?Sized, TValue>(
    items: &mut IdVec<TMarker, MaybeUninit<TValue>>,
    desired_size: usize,
) {
    while items.len() < desired_size {
        items.push(MaybeUninit::uninit());
    }
}

impl<TMarker: ?Sized, TValue> Default for IdField<TMarker, TValue> {
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
            let items_actual = transmute::<
                &IdVec<TMarker, std::mem::MaybeUninit<TValue>>,
                &IdVec<TMarker, TValue>,
            >(&self.items);
            items_actual.index(index)
        }
    }
}

impl<TMarker: ?Sized, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> IndexMut<I>
    for IdField<TMarker, TValue>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        unsafe {
            let items_actual = transmute::<
                &mut IdVec<TMarker, std::mem::MaybeUninit<TValue>>,
                &mut IdVec<TMarker, TValue>,
            >(&mut self.items);
            items_actual.index_mut(index)
        }
    }
}
