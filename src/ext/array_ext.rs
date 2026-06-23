use crate::{IdArray, internal::Sealed};

/// Brand-typed [`IdArray`] views and conversions for `[TValue; N]`.
pub trait ArrayExt<TValue, const N: usize>: Sealed {
    /// Borrows the array as a brand-typed [`IdArray`].
    fn as_id_array<TBrand: ?Sized>(&self) -> &IdArray<TBrand, TValue, N>;

    /// Mutably borrows the array as a brand-typed [`IdArray`].
    fn as_mut_id_array<TBrand: ?Sized>(&mut self) -> &mut IdArray<TBrand, TValue, N>;

    /// Converts the array into a brand-typed [`IdArray`].
    fn into_id_array<TBrand: ?Sized>(self) -> IdArray<TBrand, TValue, N>;
}

impl<TValue, const N: usize> ArrayExt<TValue, N> for [TValue; N] {
    fn as_id_array<TBrand: ?Sized>(&self) -> &IdArray<TBrand, TValue, N> {
        IdArray::from_array_ref(self)
    }

    fn as_mut_id_array<TBrand: ?Sized>(&mut self) -> &mut IdArray<TBrand, TValue, N> {
        IdArray::from_mut_array(self)
    }

    fn into_id_array<TBrand: ?Sized>(self) -> IdArray<TBrand, TValue, N> {
        IdArray::from_array(self)
    }
}
