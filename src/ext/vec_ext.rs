use crate::{internal::Sealed, IdVec};

pub trait VecExt<TValue>: Sealed {
    fn as_id_vec<TMarker>(&self) -> &IdVec<TMarker, TValue>;

    fn as_mut_id_vec<TMarker>(&mut self) -> &mut IdVec<TMarker, TValue>;

    fn to_id_vec<TMarker>(self) -> IdVec<TMarker, TValue>;
}

impl<TValue> VecExt<TValue> for Vec<TValue> {
    fn as_id_vec<TMarker>(&self) -> &IdVec<TMarker, TValue> {
        IdVec::from_vec_ref(self)
    }

    fn as_mut_id_vec<TMarker>(&mut self) -> &mut IdVec<TMarker, TValue> {
        IdVec::from_mut_vec(self)
    }

    fn to_id_vec<TMarker>(self) -> IdVec<TMarker, TValue> {
        IdVec::from_vec(self)
    }
}
