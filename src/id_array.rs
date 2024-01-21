use crate::{IdSlice, IdSliceIndex};
use std::{
    borrow::{Borrow, BorrowMut},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct IdArray<TMarker: ?Sized, TValue, const N: usize> {
    phantom: PhantomData<TMarker>,
    repr: [TValue; N],
}

impl<TMarker, TValue, const N: usize> IdArray<TMarker, TValue, N> {
    pub const fn from_array(repr: [TValue; N]) -> Self {
        IdArray {
            phantom: PhantomData,
            repr,
        }
    }

    pub const fn from_array_ref(repr: &[TValue; N]) -> &Self {
        unsafe { transmute(repr) }
    }

    pub fn from_mut_array(repr: &mut [TValue; N]) -> &mut Self {
        unsafe { transmute(repr) }
    }

    pub fn as_mut_id_slice(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(&mut self.repr)
    }

    pub const fn as_id_slice(&self) -> &IdSlice<TMarker, TValue> {
        IdSlice::from_slice(&self.repr)
    }
}

impl<TMarker, TValue, const N: usize> AsRef<IdSlice<TMarker, TValue>>
    for IdArray<TMarker, TValue, N>
{
    fn as_ref(&self) -> &IdSlice<TMarker, TValue> {
        self.as_id_slice()
    }
}

impl<TMarker, TValue, const N: usize> AsMut<IdSlice<TMarker, TValue>>
    for IdArray<TMarker, TValue, N>
{
    fn as_mut(&mut self) -> &mut IdSlice<TMarker, TValue> {
        self.as_mut_id_slice()
    }
}

impl<TMarker, TValue, const N: usize> Borrow<IdSlice<TMarker, TValue>>
    for IdArray<TMarker, TValue, N>
{
    fn borrow(&self) -> &IdSlice<TMarker, TValue> {
        IdSlice::from_slice(self.repr.borrow())
    }
}

impl<TMarker, TValue, const N: usize> BorrowMut<IdSlice<TMarker, TValue>>
    for IdArray<TMarker, TValue, N>
{
    fn borrow_mut(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(self.repr.borrow_mut())
    }
}

impl<TMarker, TValue, const N: usize> Clone for IdArray<TMarker, TValue, N>
where
    [TValue; N]: Clone,
{
    fn clone(&self) -> Self {
        IdArray::from_array(self.repr.clone())
    }
}

impl<TMarker, TValue, const N: usize> Copy for IdArray<TMarker, TValue, N> where [TValue; N]: Copy {}

impl<TMarker, TValue, const N: usize> Default for IdArray<TMarker, TValue, N>
where
    [TValue; N]: Default,
{
    fn default() -> Self {
        IdArray::from_array(Default::default())
    }
}

impl<TMarker, TValue, const N: usize> Eq for IdArray<TMarker, TValue, N> where [TValue; N]: PartialEq
{}

impl<TMarker, TValue, const N: usize> From<[TValue; N]> for IdArray<TMarker, TValue, N> {
    fn from(value: [TValue; N]) -> Self {
        IdArray::from_array(value)
    }
}

impl<TMarker, TValue, const N: usize> Hash for IdArray<TMarker, TValue, N>
where
    [TValue; N]: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.repr.hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute(data) };
        <[TValue; N] as Hash>::hash_slice(data, state)
    }
}

impl<TMarker, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>, const N: usize> Index<I>
    for IdArray<TMarker, TValue, N>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self.as_id_slice())
    }
}

impl<TMarker, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>, const N: usize> IndexMut<I>
    for IdArray<TMarker, TValue, N>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self.as_mut_id_slice())
    }
}

impl<TMarker, TValueA, TValueB, const N: usize> PartialEq<IdArray<TMarker, TValueB, N>>
    for IdArray<TMarker, TValueA, N>
where
    [TValueA; N]: PartialEq<[TValueB; N]>,
{
    fn eq(&self, other: &IdArray<TMarker, TValueB, N>) -> bool {
        self.repr.eq(&other.repr)
    }
}
