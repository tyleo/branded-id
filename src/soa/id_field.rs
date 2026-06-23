use crate::{
    Id, IdVec, Scalar,
    soa::{IdFieldIter, IdFieldIterMut, IdStruct},
};
use std::{mem, mem::MaybeUninit, ptr::write_bytes};

/// A sparse, columnar data store keyed by typed ids from an
/// [`IdStruct`].
///
/// Liveness is tracked externally by the paired [`IdStruct`], so this
/// is a thin per-id store backed by uninitialized storage:
/// [`retain`](Self::retain) a value *after* retaining the id, and
/// [`release`](Self::release) it *before* releasing the id. Reading a slot
/// that has not been retained is undefined behavior, which is why the
/// accessors are `unsafe`.
///
/// Because liveness lives in the paired [`IdStruct`], dropping an `IdField`
/// does not drop the values still retained in it; call
/// [`clear`](Self::clear) or [`release_all`](Self::release_all) first to
/// avoid leaking them. Leaking is safe, just rarely intended.
pub struct IdField<TBrand: ?Sized, TValue> {
    items: IdVec<TBrand, MaybeUninit<TValue>>,
}

impl<TBrand: ?Sized, TValue> IdField<TBrand, TValue> {
    /// Creates a new, empty field that reserves no storage up front.
    pub const fn new() -> Self {
        Self {
            items: IdVec::new(),
        }
    }

    /// Creates a new, empty field with room for `capacity` ids reserved up
    /// front. Like [`Vec::with_capacity`], this reserves storage without
    /// populating it, so [`reserved_count`](Self::reserved_count) stays 0.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: IdVec::from_vec(Vec::with_capacity(capacity)),
        }
    }

    /// Drops the value for every id retained by `ids`, then resets the
    /// field to empty, releasing its storage.
    ///
    /// `ids` supplies the liveness the field itself does not track, so the
    /// live values are dropped rather than leaked. See
    /// [`release_all`](Self::release_all) to drop the values but keep the
    /// reserved storage.
    ///
    /// # Safety
    /// `ids` must be the id pool this field is paired with, in sync with
    /// it: every id retained by `ids` must have a value `retain`'d in this
    /// field that has not since been released.
    pub unsafe fn clear<TNum: Scalar>(&mut self, ids: &IdStruct<TBrand, TNum>) {
        // SAFETY: `ids` must be the in-sync pool for this field.
        unsafe { self.release_all(ids) };
        self.items.as_mut_vec().clear();
    }

    /// # Safety
    /// A value must be `retain`'d at the id for `get` to be safe to call.
    pub unsafe fn get(&self, id: impl Id<Brand = TBrand>) -> &TValue {
        let item = &self.items[id.to_usize_id()];
        unsafe { &*item.as_ptr() }
    }

    /// # Safety
    /// A value must be `retain`'d at the id for `get_mut` to be safe to call.
    pub unsafe fn get_mut(&mut self, id: impl Id<Brand = TBrand>) -> &mut TValue {
        let item = &mut self.items[id.to_usize_id()];
        unsafe { &mut *item.as_mut_ptr() }
    }

    /// Whether the backing storage reserves a slot for `id`, not whether
    /// a value has actually been written there.
    pub fn is_reserved(&self, id: impl Id<Brand = TBrand>) -> bool {
        id.to_usize_id().to_usize() < self.reserved_count()
    }

    /// Iterates the values for every id retained by `ids`, yielding shared
    /// references in the order `ids` iterates.
    ///
    /// # Safety
    /// `ids` must be the id pool this field is paired with, in sync with
    /// it: every id retained by `ids` must have a value `retain`'d in this
    /// field that has not since been released.
    pub unsafe fn iter<'a, TNum: Scalar>(
        &'a self,
        ids: &'a IdStruct<TBrand, TNum>,
    ) -> IdFieldIter<'a, TNum::Id<TBrand>, TValue> {
        IdFieldIter::new(self.items.as_vec(), ids.into_iter())
    }

    /// Iterates the values for every id retained by `ids`, yielding mutable
    /// references in the order `ids` iterates.
    ///
    /// # Safety
    /// `ids` must be the id pool this field is paired with, in sync with
    /// it: every id retained by `ids` must have a value `retain`'d in this
    /// field that has not since been released.
    pub unsafe fn iter_mut<'a, TNum: Scalar>(
        &'a mut self,
        ids: &'a IdStruct<TBrand, TNum>,
    ) -> IdFieldIterMut<'a, TNum::Id<TBrand>, TValue> {
        let len = self.items.len();
        let items = self.items.as_mut_vec().as_mut_ptr();
        IdFieldIterMut::new(items, len, ids.into_iter())
    }

    /// # Safety
    /// A value must be `retain`'d at the id for `release` to be safe to call.
    pub unsafe fn release(&mut self, id: impl Id<Brand = TBrand>) {
        let item = &mut self.items[id.to_usize_id()];
        unsafe { MaybeUninit::assume_init_drop(item) }
    }

    /// Drops the value for every id retained by `ids`, leaving the slots
    /// reserved. Unlike [`clear`](Self::clear), this does not shrink the
    /// backing storage.
    ///
    /// # Safety
    /// `ids` must be the id pool this field is paired with, in sync with
    /// it: every id retained by `ids` must have a value `retain`'d in this
    /// field that has not since been released.
    pub unsafe fn release_all<TNum: Scalar>(&mut self, ids: &IdStruct<TBrand, TNum>) {
        for id in ids {
            // SAFETY: by contract, every id live in `ids` has a value here.
            unsafe { self.release(id) };
        }
    }

    /// Like [`release_all`](Self::release_all), but also clobbers each
    /// dropped slot's backing bytes with zeros.
    ///
    /// # Safety
    /// `ids` must be the id pool this field is paired with, in sync with
    /// it: every id retained by `ids` must have a value `retain`'d in this
    /// field that has not since been released.
    pub unsafe fn release_all_zeroed<TNum: Scalar>(&mut self, ids: &IdStruct<TBrand, TNum>) {
        for id in ids {
            // SAFETY: by contract, every id live in `ids` has a value here.
            unsafe { self.release_zeroed(id) };
        }
    }

    /// Drops the value at `id`, then clobbers its backing bytes with zeros.
    /// This does not reinitialize the slot; reading it afterward is still
    /// undefined behavior.
    ///
    /// # Safety
    /// A value must be `retain`'d at the id for `release_zeroed` to be safe
    /// to call.
    pub unsafe fn release_zeroed(&mut self, id: impl Id<Brand = TBrand>) {
        let item = &mut self.items[id.to_usize_id()];
        unsafe { MaybeUninit::assume_init_drop(item) }

        let size = mem::size_of::<TValue>();
        if size != 0 {
            let p = item.as_mut_ptr() as *mut u8;
            unsafe { write_bytes(p, 0, size) }
        }
    }

    /// Ensures at least `count` id slots are reserved (so
    /// [`reserved_count`](Self::reserved_count) is at least `count`),
    /// growing with uninitialized storage if needed. Never shrinks.
    pub fn reserve(&mut self, count: usize) {
        while self.items.len() < count {
            self.items.push(MaybeUninit::uninit());
        }
    }

    /// The number of id slots currently reserved (not necessarily written).
    pub fn reserved_count(&self) -> usize {
        self.items.len()
    }

    /// Reserves storage up through `id` and writes `value` into its slot,
    /// returning a reference to it. Unlike [`set`](Self::set) this does not
    /// drop a previous value, so it must only be called for an `id` whose slot
    /// is not currently retained.
    pub fn retain(&mut self, id: impl Id<Brand = TBrand>, value: TValue) -> &mut TValue {
        let id = id.to_usize_id();
        self.reserve(id.to_usize() + 1);
        self.items[id].write(value)
    }

    /// # Safety
    /// A value must be `retain`'d at the id for `set` to be safe to call.
    pub unsafe fn set(&mut self, id: impl Id<Brand = TBrand>, value: TValue) -> &mut TValue {
        let item = &mut self.items[id.to_usize_id()];
        unsafe { MaybeUninit::assume_init_drop(item) }
        item.write(value)
    }
}

impl<TBrand: ?Sized, TValue> Default for IdField<TBrand, TValue> {
    fn default() -> Self {
        Self::new()
    }
}
