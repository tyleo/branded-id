use crate::{IdVec, internal::Sealed};

pub trait VecExt<TValue>: Sealed {
    fn as_id_vec<TBrand: ?Sized>(&self) -> &IdVec<TBrand, TValue>;

    fn as_mut_id_vec<TBrand: ?Sized>(&mut self) -> &mut IdVec<TBrand, TValue>;

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
