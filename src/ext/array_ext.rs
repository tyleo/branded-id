use crate::{IdArray, internal::Sealed};

pub trait ArrayExt<TValue, const N: usize>: Sealed {
    fn as_id_array<TBrand: ?Sized>(&self) -> &IdArray<TBrand, TValue, N>;

    fn as_mut_id_array<TBrand: ?Sized>(&mut self) -> &mut IdArray<TBrand, TValue, N>;

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
