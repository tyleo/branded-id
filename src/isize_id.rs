use crate::internal;

internal::scalar_id! { IsizeId, isize, from_isize, to_isize }

impl<TBrand: ?Sized> IsizeId<TBrand> {
    /// Returns the id advanced by `value`, keeping the brand.
    ///
    /// # Panics
    /// Panics if the addition overflows `isize`.
    pub const fn offset(self, value: isize) -> IsizeId<TBrand> {
        IsizeId::from_isize(self.to_isize() + value)
    }
}
