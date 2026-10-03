use crate::{DisplayIds, Id, IdSlice, internal::Sealed};

/// Brand-typed [`IdSlice`] views and id printing for `[TValue]`.
pub trait SliceExt<TValue>: Sealed {
    /// Borrows the slice as a brand-typed [`IdSlice`].
    fn as_id_slice<TBrand: ?Sized>(&self) -> &IdSlice<TBrand, TValue>;

    /// Mutably borrows the slice as a brand-typed [`IdSlice`].
    fn as_mut_id_slice<TBrand: ?Sized>(&mut self) -> &mut IdSlice<TBrand, TValue>;

    /// Returns a [`DisplayIds`] that prints the ids as `[1, 2]`.
    fn display_ids(&self) -> DisplayIds<'_, TValue>
    where
        TValue: Id;
}

impl<TValue> SliceExt<TValue> for [TValue] {
    fn as_id_slice<TBrand: ?Sized>(&self) -> &IdSlice<TBrand, TValue> {
        IdSlice::from_slice(self)
    }

    fn as_mut_id_slice<TBrand: ?Sized>(&mut self) -> &mut IdSlice<TBrand, TValue> {
        IdSlice::from_mut_slice(self)
    }

    fn display_ids(&self) -> DisplayIds<'_, TValue>
    where
        TValue: Id,
    {
        DisplayIds::new(self)
    }
}
