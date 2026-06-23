use crate::{IdVec, UsizeId};
use std::{mem, mem::MaybeUninit, ptr::write_bytes};

/// A sparse, columnar data store keyed by typed ids from a
/// [`UsizeIdStruct`].
///
/// Liveness is tracked externally by the paired [`UsizeIdStruct`], so this
/// is a thin per-id store backed by uninitialized storage: [`retain`] a
/// value *after* retaining the id, and [`release`] it *before* releasing
/// the id. Reading a slot that has not been retained is undefined
/// behavior, which is why the accessors are `unsafe`.
pub struct IdField<TMarker: ?Sized, TValue> {
    items: IdVec<TMarker, MaybeUninit<TValue>>,
}

impl<TMarker: ?Sized, TValue> IdField<TMarker, TValue> {
    pub const fn new() -> Self {
        Self {
            items: IdVec::new(),
        }
    }

    /// Creates a new, empty field with room for `capacity` ids reserved up
    /// front. Like [`Vec::with_capacity`], this reserves storage without
    /// populating it, so [`reserved_count`](Self::reserved_count) stays 0.
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            items: IdVec::from_vec(Vec::with_capacity(capacity)),
        }
    }

    /// Resets the field to empty.
    ///
    /// This does **not** drop the stored values — `MaybeUninit` has no
    /// destructor and the field does not know which slots are live, so any
    /// still-retained value is leaked. [`release`](Self::release) every
    /// retained value first.
    pub fn clear(&mut self) {
        self.items.as_mut_vec().clear();
    }

    /// Ensures the field reserves at least `count` id slots, growing with
    /// uninitialized storage if needed. Never shrinks.
    pub fn ensure_count(&mut self, count: usize) {
        while self.items.len() < count {
            self.items.push(MaybeUninit::uninit());
        }
    }

    /// # Safety
    /// A value must be `retain`'d at the id for `get` to be safe to call.
    pub unsafe fn get(&self, id: UsizeId<TMarker>) -> &TValue {
        let item = &self.items[id];
        unsafe { &*item.as_ptr() }
    }

    /// # Safety
    /// A value must be `retain`'d at the id for `get_mut` to be safe to call.
    pub unsafe fn get_mut(&mut self, id: UsizeId<TMarker>) -> &mut TValue {
        let item = &mut self.items[id];
        unsafe { &mut *item.as_mut_ptr() }
    }

    /// Whether the backing storage reserves a slot for `id` — *not* whether
    /// a value has actually been written there.
    pub fn is_reserved(&self, id: UsizeId<TMarker>) -> bool {
        id.to_usize() < self.reserved_count()
    }

    /// # Safety
    /// A value must be `retain`'d at the id for `release` to be safe to call.
    pub unsafe fn release(&mut self, id: UsizeId<TMarker>) {
        let item = &mut self.items[id];
        unsafe { MaybeUninit::assume_init_drop(item) }
    }

    /// Drops the value at `id`, then clobbers its backing bytes with zeros.
    /// This does not reinitialize the slot; reading it afterward is still
    /// undefined behavior.
    ///
    /// # Safety
    /// A value must be `retain`'d at the id for `release_zeroed` to be safe
    /// to call.
    pub unsafe fn release_zeroed(&mut self, id: UsizeId<TMarker>) {
        let item = &mut self.items[id];
        unsafe { MaybeUninit::assume_init_drop(item) }

        let size = mem::size_of::<TValue>();
        if size != 0 {
            let p = item.as_mut_ptr() as *mut u8;
            unsafe { write_bytes(p, 0, size) }
        }
    }

    /// The number of id slots currently reserved (not necessarily written).
    pub fn reserved_count(&self) -> usize {
        self.items.len()
    }

    pub fn retain(&mut self, id: UsizeId<TMarker>, value: TValue) -> &mut TValue {
        self.ensure_count(id.to_usize() + 1);
        self.items[id].write(value)
    }

    /// # Safety
    /// A value must be `retain`'d at the id for `set` to be safe to call.
    pub unsafe fn set(&mut self, id: UsizeId<TMarker>, value: TValue) -> &mut TValue {
        let item = &mut self.items[id];
        unsafe { MaybeUninit::assume_init_drop(item) }
        item.write(value)
    }
}

impl<TMarker: ?Sized, TValue> Default for IdField<TMarker, TValue> {
    fn default() -> Self {
        Self::new()
    }
}
