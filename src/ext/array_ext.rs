use crate::{internal::Sealed, IdArray};

pub trait ArrayExt<TValue, const N: usize>: Sealed {
    fn as_id_array<TMarker>(&self) -> &IdArray<TMarker, TValue, N>;

    fn as_mut_id_array<TMarker>(&mut self) -> &mut IdArray<TMarker, TValue, N>;

    fn to_id_array<TMarker>(self) -> IdArray<TMarker, TValue, N>;
}

impl<TValue, const N: usize> ArrayExt<TValue, N> for [TValue; N] {
    fn as_id_array<TMarker>(&self) -> &IdArray<TMarker, TValue, N> {
        IdArray::from_array_ref(self)
    }

    fn as_mut_id_array<TMarker>(&mut self) -> &mut IdArray<TMarker, TValue, N> {
        IdArray::from_mut_array(self)
    }

    fn to_id_array<TMarker>(self) -> IdArray<TMarker, TValue, N> {
        IdArray::from_array(self)
    }
}
