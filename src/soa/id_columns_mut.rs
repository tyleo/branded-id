use crate::{
    Scalar, UsizeId,
    soa::{IdColumns, IdField, IdRemap, IdStruct},
};

/// Columns that are all mutable, so an
/// [`IdStructViewMut`](super::IdStructViewMut) can add and remove rows: one
/// `&mut IdField` or a tuple of up to eight.
///
/// The trait is sealed. Its hidden members serve the view.
pub trait IdColumnsMut<TBrand: ?Sized>: IdColumns<TBrand> {
    /// A row's values, owned: what an add takes and a remove hands back.
    type Values;

    #[doc(hidden)]
    fn retain(&mut self, id: UsizeId<TBrand>, values: Self::Values);

    /// # Safety
    /// `id` has a value in every column.
    #[doc(hidden)]
    unsafe fn take(&mut self, id: UsizeId<TBrand>) -> Self::Values;

    /// # Safety
    /// As [`take`](Self::take).
    #[doc(hidden)]
    unsafe fn take_zeroed(&mut self, id: UsizeId<TBrand>) -> Self::Values;

    /// # Safety
    /// As [`IdField::gc`], for every column.
    #[doc(hidden)]
    unsafe fn gc<TNum: Scalar>(&mut self, remap: &IdRemap<TBrand, TNum>);

    /// # Safety
    /// As [`IdField::clear`], for every column.
    #[doc(hidden)]
    unsafe fn clear<TNum: Scalar>(&mut self, ids: &IdStruct<TBrand, TNum>);
}

impl<TBrand: ?Sized, TValue> IdColumnsMut<TBrand> for &mut IdField<TBrand, TValue> {
    type Values = TValue;

    fn retain(&mut self, id: UsizeId<TBrand>, values: Self::Values) {
        IdField::retain(self, id, values);
    }

    unsafe fn take(&mut self, id: UsizeId<TBrand>) -> Self::Values {
        // SAFETY: forwarded to the caller.
        unsafe { IdField::take(self, id) }
    }

    unsafe fn take_zeroed(&mut self, id: UsizeId<TBrand>) -> Self::Values {
        // SAFETY: forwarded to the caller.
        unsafe { IdField::take_zeroed(self, id) }
    }

    unsafe fn gc<TNum: Scalar>(&mut self, remap: &IdRemap<TBrand, TNum>) {
        // SAFETY: forwarded to the caller.
        unsafe { IdField::gc(self, remap) }
    }

    unsafe fn clear<TNum: Scalar>(&mut self, ids: &IdStruct<TBrand, TNum>) {
        // SAFETY: forwarded to the caller.
        unsafe { IdField::clear(self, ids) }
    }
}
