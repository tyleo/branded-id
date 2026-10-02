use crate::{UsizeId, internal::Sealed, soa::IdField};
use std::mem::MaybeUninit;

/// The columns an [`IdStructView`](super::IdStructView) reads beside its
/// [`IdStruct`](super::IdStruct): one `&IdField` or `&mut IdField`, or a tuple
/// of up to eight columns mixing the two.
///
/// A row holds one reference per column. A shared column always reads as
/// `&T`, and those references can outlive the view. A mutable column reads as
/// `&T` through a shared borrow of the view and as `&mut T` through a mutable
/// one.
///
/// The trait is sealed. Its hidden members serve the views.
pub trait IdColumns<TBrand: ?Sized>: Sealed {
    /// A row read through a shared borrow of the view.
    type Ref<'r>
    where
        Self: 'r;

    /// A row read through a mutable borrow of the view.
    type Mut<'r>
    where
        Self: 'r;

    #[doc(hidden)]
    type RawRef: Copy;

    #[doc(hidden)]
    type RawMut: Copy;

    #[doc(hidden)]
    fn raw_ref(&self) -> Self::RawRef;

    #[doc(hidden)]
    fn raw_mut(&mut self) -> Self::RawMut;

    /// # Safety
    /// `raw` comes from these columns, which outlive `'r` unchanged, and `id`
    /// has a value in every column.
    #[doc(hidden)]
    unsafe fn row_ref<'r>(raw: Self::RawRef, id: UsizeId<TBrand>) -> Self::Ref<'r>
    where
        Self: 'r;

    /// # Safety
    /// As [`row_ref`](Self::row_ref), and no other row for `id` from the same
    /// `raw` is alive during `'r`.
    #[doc(hidden)]
    unsafe fn row_mut<'r>(raw: Self::RawMut, id: UsizeId<TBrand>) -> Self::Mut<'r>
    where
        Self: 'r;
}

impl<TBrand: ?Sized, TValue> Sealed for &IdField<TBrand, TValue> {}

impl<'a, TBrand: ?Sized, TValue> IdColumns<TBrand> for &'a IdField<TBrand, TValue> {
    type Ref<'r>
        = &'a TValue
    where
        Self: 'r;

    type Mut<'r>
        = &'a TValue
    where
        Self: 'r;

    type RawRef = &'a IdField<TBrand, TValue>;

    type RawMut = &'a IdField<TBrand, TValue>;

    fn raw_ref(&self) -> Self::RawRef {
        *self
    }

    fn raw_mut(&mut self) -> Self::RawMut {
        *self
    }

    unsafe fn row_ref<'r>(raw: Self::RawRef, id: UsizeId<TBrand>) -> Self::Ref<'r>
    where
        Self: 'r,
    {
        // SAFETY: forwarded to the caller.
        unsafe { raw.get(id) }
    }

    unsafe fn row_mut<'r>(raw: Self::RawMut, id: UsizeId<TBrand>) -> Self::Mut<'r>
    where
        Self: 'r,
    {
        // SAFETY: forwarded to the caller.
        unsafe { raw.get(id) }
    }
}

impl<TBrand: ?Sized, TValue> Sealed for &mut IdField<TBrand, TValue> {}

impl<TBrand: ?Sized, TValue> IdColumns<TBrand> for &mut IdField<TBrand, TValue> {
    type Ref<'r>
        = &'r TValue
    where
        Self: 'r;

    type Mut<'r>
        = &'r mut TValue
    where
        Self: 'r;

    type RawRef = *const [MaybeUninit<TValue>];

    type RawMut = *mut [MaybeUninit<TValue>];

    fn raw_ref(&self) -> Self::RawRef {
        self.raw_items()
    }

    fn raw_mut(&mut self) -> Self::RawMut {
        self.raw_items_mut()
    }

    unsafe fn row_ref<'r>(raw: Self::RawRef, id: UsizeId<TBrand>) -> Self::Ref<'r>
    where
        Self: 'r,
    {
        let index = id.to_usize();
        assert!(index < raw.len(), "id is out of range for this field");

        // SAFETY: `index` is in range, and the caller vouches that the slot
        // holds a value that stays unchanged for `'r`.
        unsafe { (*raw.cast::<MaybeUninit<TValue>>().add(index)).assume_init_ref() }
    }

    unsafe fn row_mut<'r>(raw: Self::RawMut, id: UsizeId<TBrand>) -> Self::Mut<'r>
    where
        Self: 'r,
    {
        let index = id.to_usize();
        assert!(index < raw.len(), "id is out of range for this field");

        // SAFETY: `index` is in range, and the caller vouches that the slot
        // holds a value no other live row borrows.
        unsafe { (*raw.cast::<MaybeUninit<TValue>>().add(index)).assume_init_mut() }
    }
}
