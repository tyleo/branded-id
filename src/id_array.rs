use crate::{IdSlice, IdSliceIndex};
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::{Index, IndexMut},
};

pub struct IdArray<TMarker: ?Sized, TValue, const N: usize> {
    phantom: PhantomData<TMarker>,
    repr: [TValue; N],
}

impl<TMarker, TValue, const N: usize> IdArray<TMarker, TValue, N> {
    pub const fn as_array(&self) -> &[TValue; N] {
        &self.repr
    }

    pub fn as_mut_array(&mut self) -> &mut [TValue; N] {
        &mut self.repr
    }

    pub fn as_mut_id_slice(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(self.as_mut_array())
    }

    pub const fn as_id_slice(&self) -> &IdSlice<TMarker, TValue> {
        IdSlice::from_slice(self.as_array())
    }

    pub const fn from_array(repr: [TValue; N]) -> Self {
        Self {
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

    pub fn into_array(self) -> [TValue; N] {
        self.repr
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
        IdSlice::from_slice(self.as_array().borrow())
    }
}

impl<TMarker, TValue, const N: usize> BorrowMut<IdSlice<TMarker, TValue>>
    for IdArray<TMarker, TValue, N>
{
    fn borrow_mut(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(self.as_mut_array().borrow_mut())
    }
}

impl<TMarker, TValue, const N: usize> Clone for IdArray<TMarker, TValue, N>
where
    [TValue; N]: Clone,
{
    fn clone(&self) -> Self {
        Self::from_array(self.as_array().clone())
    }
}

impl<TMarker, TValue, const N: usize> Copy for IdArray<TMarker, TValue, N> where [TValue; N]: Copy {}

impl<TMarker, TValue: Debug, const N: usize> Debug for IdArray<TMarker, TValue, N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self.as_id_slice(), f)
    }
}

impl<TMarker, TValue, const N: usize> Default for IdArray<TMarker, TValue, N>
where
    [TValue; N]: Default,
{
    fn default() -> Self {
        Self::from_array(Default::default())
    }
}

impl<TMarker, TValue, const N: usize> Eq for IdArray<TMarker, TValue, N> where [TValue; N]: PartialEq
{}

impl<TMarker, TValue, const N: usize> From<[TValue; N]> for IdArray<TMarker, TValue, N> {
    fn from(value: [TValue; N]) -> Self {
        Self::from_array(value)
    }
}

impl<TMarker, TValue, const N: usize> Hash for IdArray<TMarker, TValue, N>
where
    [TValue; N]: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_array().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute(data) };
        <[TValue; N]>::hash_slice(data, state)
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

#[allow(clippy::into_iter_on_ref)]
impl<'a, TMarker, TValue, const N: usize> IntoIterator for &'a IdArray<TMarker, TValue, N> {
    type Item = <&'a [TValue; N] as IntoIterator>::Item;
    type IntoIter = <&'a [TValue; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.as_array().into_iter()
    }
}

#[allow(clippy::into_iter_on_ref)]
impl<'a, TMarker, TValue, const N: usize> IntoIterator for &'a mut IdArray<TMarker, TValue, N> {
    type Item = <&'a mut [TValue; N] as IntoIterator>::Item;
    type IntoIter = <&'a mut [TValue; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_array().into_iter()
    }
}

impl<TMarker, TValue, const N: usize> IntoIterator for IdArray<TMarker, TValue, N> {
    type Item = <[TValue; N] as IntoIterator>::Item;
    type IntoIter = <[TValue; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

impl<TMarker, TValue: Ord, const N: usize> Ord for IdArray<TMarker, TValue, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_array().cmp(other.as_array())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_array(self.into_array().max(other.into_array()))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_array(self.into_array().min(other.into_array()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_array(self.into_array().clamp(min.into_array(), max.into_array()))
    }
}

impl<'a, TMarker, TValueA, TValueB, const N: usize> PartialEq<&'a IdSlice<TMarker, TValueB>>
    for IdArray<TMarker, TValueA, N>
where
    [TValueA; N]: PartialEq<&'a [TValueB]>,
{
    fn eq(&self, other: &&'a IdSlice<TMarker, TValueB>) -> bool {
        self.as_array().eq(&other.as_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&'a IdSlice<TMarker, TValueB>) -> bool {
        self.as_array().ne(&other.as_slice())
    }
}

impl<'a, TMarker, TValueA, TValueB, const N: usize> PartialEq<&'a mut IdSlice<TMarker, TValueB>>
    for IdArray<TMarker, TValueA, N>
where
    [TValueA; N]: PartialEq<&'a mut [TValueB]>,
{
    fn eq(&self, other: &&'a mut IdSlice<TMarker, TValueB>) -> bool {
        let other: &&'a mut _ = unsafe { transmute(other) };
        self.as_array().eq(other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&'a mut IdSlice<TMarker, TValueB>) -> bool {
        let other: &&'a mut _ = unsafe { transmute(other) };
        self.as_array().ne(other)
    }
}

impl<TMarker, TValueA, TValueB, const N: usize> PartialEq<IdSlice<TMarker, TValueB>>
    for IdArray<TMarker, TValueA, N>
where
    [TValueA; N]: PartialEq<[TValueB]>,
{
    fn eq(&self, other: &IdSlice<TMarker, TValueB>) -> bool {
        self.as_array().eq(other.as_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdSlice<TMarker, TValueB>) -> bool {
        self.as_array().ne(other.as_slice())
    }
}

impl<TMarker, TValueA, TValueB, const N: usize> PartialEq<IdArray<TMarker, TValueB, N>>
    for IdArray<TMarker, TValueA, N>
where
    [TValueA; N]: PartialEq<[TValueB; N]>,
{
    fn eq(&self, other: &IdArray<TMarker, TValueB, N>) -> bool {
        self.as_array().eq(other.as_array())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdArray<TMarker, TValueB, N>) -> bool {
        self.as_array().ne(other.as_array())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker, TValue: PartialOrd, const N: usize> PartialOrd for IdArray<TMarker, TValue, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_array().partial_cmp(other.as_array())
    }

    fn lt(&self, other: &Self) -> bool {
        self.as_array().lt(other.as_array())
    }

    fn le(&self, other: &Self) -> bool {
        self.as_array().le(other.as_array())
    }

    fn gt(&self, other: &Self) -> bool {
        self.as_array().gt(other.as_array())
    }

    fn ge(&self, other: &Self) -> bool {
        self.as_array().ge(other.as_array())
    }
}
