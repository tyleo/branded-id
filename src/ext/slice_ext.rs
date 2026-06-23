use crate::{IdSlice, internal::Sealed};

/// Brand-typed [`IdSlice`] views for `[TValue]`.
pub trait SliceExt<TValue>: Sealed {
    /// Borrows the slice as a brand-typed [`IdSlice`].
    fn as_id_slice<TBrand: ?Sized>(&self) -> &IdSlice<TBrand, TValue>;

    /// Mutably borrows the slice as a brand-typed [`IdSlice`].
    fn as_mut_id_slice<TBrand: ?Sized>(&mut self) -> &mut IdSlice<TBrand, TValue>;
}

impl<TValue> SliceExt<TValue> for [TValue] {
    fn as_id_slice<TBrand: ?Sized>(&self) -> &IdSlice<TBrand, TValue> {
        IdSlice::from_slice(self)
    }

    fn as_mut_id_slice<TBrand: ?Sized>(&mut self) -> &mut IdSlice<TBrand, TValue> {
        IdSlice::from_mut_slice(self)
    }
}
