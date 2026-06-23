use crate::{IdVec, internal::Sealed};

/// Brand-typed [`IdVec`] views and conversions for `Vec<TValue>`.
pub trait VecExt<TValue>: Sealed {
    /// Borrows the vec as a brand-typed [`IdVec`].
    fn as_id_vec<TBrand: ?Sized>(&self) -> &IdVec<TBrand, TValue>;

    /// Mutably borrows the vec as a brand-typed [`IdVec`].
    fn as_mut_id_vec<TBrand: ?Sized>(&mut self) -> &mut IdVec<TBrand, TValue>;

    /// Converts the vec into a brand-typed [`IdVec`].
    fn into_id_vec<TBrand: ?Sized>(self) -> IdVec<TBrand, TValue>;
}

impl<TValue> VecExt<TValue> for Vec<TValue> {
    fn as_id_vec<TBrand: ?Sized>(&self) -> &IdVec<TBrand, TValue> {
        IdVec::from_vec_ref(self)
    }

    fn as_mut_id_vec<TBrand: ?Sized>(&mut self) -> &mut IdVec<TBrand, TValue> {
        IdVec::from_mut_vec(self)
    }

    fn into_id_vec<TBrand: ?Sized>(self) -> IdVec<TBrand, TValue> {
        IdVec::from_vec(self)
    }
}
