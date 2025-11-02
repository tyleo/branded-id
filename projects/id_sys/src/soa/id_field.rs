use crate::{IdVec, UsizeId};
use std::{mem::MaybeUninit, ptr::write_bytes};

pub struct IdField<TMarker: ?Sized, TValue> {
    items: IdVec<TMarker, MaybeUninit<TValue>>,
}

impl<TMarker: ?Sized, TValue> IdField<TMarker, TValue> {
    pub const fn new() -> Self {
        Self {
            items: IdVec::new(),
        }
    }

    pub fn retain(&mut self, id: UsizeId<TMarker>, value: TValue) -> &mut TValue {
        ensure_size(&mut self.items, id.to_usize() + 1);
        self.items[id].write(value)
    }

    /// # Safety
    /// A value must be `retain()`'d at the id for `release()` to be safe to call.
    pub unsafe fn release(&mut self, id: UsizeId<TMarker>) {
        let item = &mut self.items[id];
        unsafe { MaybeUninit::assume_init_drop(item) }
    }

    /// # Safety
    /// A value must be `retain()`'d at the id for `release()` to be safe to call.
    pub unsafe fn release_zero(&mut self, id: UsizeId<TMarker>) {
        let item = &mut self.items[id];
        unsafe { MaybeUninit::assume_init_drop(item) }

        let size = core::mem::size_of::<TValue>();
        if size != 0 {
            let p = item.as_mut_ptr() as *mut u8;
            // This does not reinitialize the value; it just clobbers the backing bytes.
            unsafe { write_bytes(p, 0, size) }
        }
    }

    /// # Safety
    /// A value must be `retain()`'d at the id for `get()` to be safe to call.
    pub unsafe fn get(&self, id: UsizeId<TMarker>) -> &TValue {
        let item = &self.items[id];
        unsafe { &*item.as_ptr() }
    }

    /// # Safety
    /// A value must be `retain()`'d at the id for `get_mut()` to be safe to call.
    pub unsafe fn get_mut(&mut self, id: UsizeId<TMarker>) -> &mut TValue {
        let item = &mut self.items[id];
        unsafe { &mut *item.as_mut_ptr() }
    }

    /// # Safety
    /// A value must be `retain()`'d at the id for `set()` to be safe to call.
    pub unsafe fn set(&mut self, id: UsizeId<TMarker>, value: TValue) -> &mut TValue {
        let item = &mut self.items[id];
        unsafe { MaybeUninit::assume_init_drop(item) }
        item.write(value)
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
