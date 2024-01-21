use crate::{IdSlice, IdSliceIndex, UsizeId};
use std::{
    borrow::{Borrow, BorrowMut},
    marker::PhantomData,
    mem::transmute,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct IdVec<TMarker: ?Sized, TValue> {
    repr: Vec<TValue>,
    phantom: PhantomData<TMarker>,
}

impl<TMarker, TValue> IdVec<TMarker, TValue> {
    pub const fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    pub const fn from_vec(vec: Vec<TValue>) -> Self {
        Self {
            repr: vec,
            phantom: PhantomData,
        }
    }

    pub fn from_mut_vec(vec: &mut Vec<TValue>) -> &mut Self {
        unsafe { transmute(vec) }
    }

    pub const fn from_vec_ref(vec: &Vec<TValue>) -> &Self {
        unsafe { transmute(vec) }
    }

    pub fn as_mut_id_slice(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(&mut self.repr)
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<TValue> {
        &mut self.repr
    }

    pub fn as_id_slice(&self) -> &IdSlice<TMarker, TValue> {
        IdSlice::from_slice(&self.repr)
    }

    pub fn as_vec(&self) -> &Vec<TValue> {
        &self.repr
    }

    pub fn is_empty(&self) -> bool {
        self.repr.is_empty()
    }

    pub fn end(&self) -> UsizeId<TMarker> {
        self.as_id_slice().end()
    }

    pub fn len(&self) -> usize {
        self.repr.len()
    }

    pub fn push(&mut self, value: TValue) -> UsizeId<TMarker> {
        let res = self.end();
        self.repr.push(value);
        res
    }

    pub fn resize(&mut self, new_len: usize, value: TValue)
    where
        TValue: Clone,
    {
        self.repr.resize(new_len, value)
    }
}

impl<TMarker, TValue> AsRef<IdSlice<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn as_ref(&self) -> &IdSlice<TMarker, TValue> {
        self.as_id_slice()
    }
}

impl<TMarker, TValue> AsMut<IdSlice<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn as_mut(&mut self) -> &mut IdSlice<TMarker, TValue> {
        self.as_mut_id_slice()
    }
}

impl<TMarker, TValue> Borrow<IdSlice<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn borrow(&self) -> &IdSlice<TMarker, TValue> {
        IdSlice::from_slice(self.repr.borrow())
    }
}

impl<TMarker, TValue> BorrowMut<IdSlice<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn borrow_mut(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(self.repr.borrow_mut())
    }
}

impl<TMarker, TValue> Default for IdVec<TMarker, TValue> {
    fn default() -> Self {
        Self::from_vec(Default::default())
    }
}

impl<TMarker, TValue> Eq for IdVec<TMarker, TValue> where Vec<TValue>: PartialEq {}

impl<TMarker, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> Index<I>
    for IdVec<TMarker, TValue>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.as_id_slice()[index]
    }
}

impl<TMarker, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> IndexMut<I>
    for IdVec<TMarker, TValue>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_mut_id_slice()[index]
    }
}

impl<TMarker, TValue> PartialEq for IdVec<TMarker, TValue>
where
    Vec<TValue>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}
