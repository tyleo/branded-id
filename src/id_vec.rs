use crate::{IdArray, IdSlice, IdSliceIndex, UsizeId};
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, DerefMut, Index, IndexMut},
};

pub struct IdVec<TMarker: ?Sized, TValue> {
    phantom: PhantomData<TMarker>,
    repr: Vec<TValue>,
}

impl<TMarker: ?Sized, TValue> IdVec<TMarker, TValue> {
    pub fn as_mut_id_slice(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(&mut self.repr)
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<TValue> {
        &mut self.repr
    }

    pub fn as_id_slice(&self) -> &IdSlice<TMarker, TValue> {
        IdSlice::from_slice(&self.repr)
    }

    pub const fn as_vec(&self) -> &Vec<TValue> {
        &self.repr
    }

    pub fn end(&self) -> UsizeId<TMarker> {
        self.as_id_slice().end()
    }

    pub fn from_mut_vec(vec: &mut Vec<TValue>) -> &mut Self {
        unsafe { transmute(vec) }
    }

    pub const fn from_vec(vec: Vec<TValue>) -> Self {
        Self {
            phantom: PhantomData,
            repr: vec,
        }
    }

    pub const fn from_vec_ref(vec: &Vec<TValue>) -> &Self {
        unsafe { transmute(vec) }
    }

    pub fn into_vec(self) -> Vec<TValue> {
        self.repr
    }

    pub fn is_empty(&self) -> bool {
        self.repr.is_empty()
    }

    pub fn len(&self) -> usize {
        self.repr.len()
    }

    pub const fn new() -> Self {
        Self::from_vec(Vec::new())
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

impl<TMarker: ?Sized, TValue> AsMut<IdSlice<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn as_mut(&mut self) -> &mut IdSlice<TMarker, TValue> {
        self.as_mut_id_slice()
    }
}

impl<TMarker: ?Sized, TValue> AsMut<IdVec<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn as_mut(&mut self) -> &mut IdVec<TMarker, TValue> {
        self
    }
}

impl<TMarker: ?Sized, TValue> AsRef<IdSlice<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn as_ref(&self) -> &IdSlice<TMarker, TValue> {
        self.as_id_slice()
    }
}

impl<TMarker: ?Sized, TValue> AsRef<IdVec<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn as_ref(&self) -> &IdVec<TMarker, TValue> {
        self
    }
}

impl<TMarker: ?Sized, TValue> Borrow<IdSlice<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn borrow(&self) -> &IdSlice<TMarker, TValue> {
        IdSlice::from_slice(self.as_vec().borrow())
    }
}

impl<TMarker: ?Sized, TValue> BorrowMut<IdSlice<TMarker, TValue>> for IdVec<TMarker, TValue> {
    fn borrow_mut(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(self.as_mut_vec().borrow_mut())
    }
}

impl<TMarker: ?Sized, TValue: Clone> Clone for IdVec<TMarker, TValue> {
    fn clone(&self) -> Self {
        Self::from_vec(self.as_vec().clone())
    }
}

impl<TMarker: ?Sized, TValue: Debug> Debug for IdVec<TMarker, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_id_slice().fmt(f)
    }
}

impl<TMarker: ?Sized, TValue> Default for IdVec<TMarker, TValue> {
    fn default() -> Self {
        Self::from_vec(Default::default())
    }
}

impl<TMarker: ?Sized, TValue> Deref for IdVec<TMarker, TValue> {
    type Target = IdSlice<TMarker, TValue>;

    fn deref(&self) -> &Self::Target {
        IdSlice::from_slice(self.as_vec().deref())
    }
}

impl<TMarker: ?Sized, TValue> DerefMut for IdVec<TMarker, TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        IdSlice::from_mut_slice(self.as_mut_vec().deref_mut())
    }
}

impl<TMarker: ?Sized, TValue> Eq for IdVec<TMarker, TValue> where TValue: PartialEq {}

impl<'a, TMarker: ?Sized, TValue> Extend<&'a TValue> for IdVec<TMarker, TValue>
where
    TValue: Copy + 'a,
{
    fn extend<T: IntoIterator<Item = &'a TValue>>(&mut self, iter: T) {
        self.as_mut_vec().extend(iter)
    }
}

impl<TMarker: ?Sized, TValue> Extend<TValue> for IdVec<TMarker, TValue> {
    fn extend<T: IntoIterator<Item = TValue>>(&mut self, iter: T) {
        self.as_mut_vec().extend(iter)
    }
}

impl<TMarker: ?Sized, TValue> From<Vec<TValue>> for IdVec<TMarker, TValue> {
    fn from(value: Vec<TValue>) -> Self {
        Self::from_vec(value)
    }
}

impl<TMarker: ?Sized, TValue> FromIterator<TValue> for IdVec<TMarker, TValue> {
    fn from_iter<T: IntoIterator<Item = TValue>>(iter: T) -> Self {
        Self::from_vec(Vec::from_iter(iter))
    }
}

impl<TMarker: ?Sized, TValue> Hash for IdVec<TMarker, TValue>
where
    Vec<TValue>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_vec().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute(data) };
        <Vec<TValue>>::hash_slice(data, state)
    }
}

impl<TMarker: ?Sized, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> Index<I>
    for IdVec<TMarker, TValue>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.as_id_slice()[index]
    }
}

impl<TMarker: ?Sized, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> IndexMut<I>
    for IdVec<TMarker, TValue>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_mut_id_slice()[index]
    }
}

impl<'a, TMarker: ?Sized, TValue> IntoIterator for &'a IdVec<TMarker, TValue> {
    type Item = <&'a Vec<TValue> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<TValue> as IntoIterator>::IntoIter;

    #[allow(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_vec().into_iter()
    }
}

impl<'a, TMarker: ?Sized, TValue> IntoIterator for &'a mut IdVec<TMarker, TValue> {
    type Item = <&'a mut Vec<TValue> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<TValue> as IntoIterator>::IntoIter;

    #[allow(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_vec().into_iter()
    }
}

impl<TMarker: ?Sized, TValue> IntoIterator for IdVec<TMarker, TValue> {
    type Item = <Vec<TValue> as IntoIterator>::Item;
    type IntoIter = <Vec<TValue> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<TMarker: ?Sized, TValue: Ord> Ord for IdVec<TMarker, TValue> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_vec().cmp(other.as_vec())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_vec(self.into_vec().max(other.into_vec()))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_vec(self.into_vec().min(other.into_vec()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_vec(self.into_vec().clamp(min.into_vec(), max.into_vec()))
    }
}

impl<TMarker: ?Sized, TValueA, TValueB> PartialEq<&IdSlice<TMarker, TValueB>>
    for IdVec<TMarker, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &&IdSlice<TMarker, TValueB>) -> bool {
        self.as_id_slice().eq(*other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&IdSlice<TMarker, TValueB>) -> bool {
        self.as_id_slice().ne(*other)
    }
}

impl<TMarker: ?Sized, TValueA, TValueB, const N: usize> PartialEq<&IdArray<TMarker, TValueB, N>>
    for IdVec<TMarker, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &&IdArray<TMarker, TValueB, N>) -> bool {
        self.as_id_slice().eq(other.as_id_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&IdArray<TMarker, TValueB, N>) -> bool {
        self.as_id_slice().ne(other.as_id_slice())
    }
}

impl<TMarker: ?Sized, TValueA, TValueB> PartialEq<&mut IdSlice<TMarker, TValueB>>
    for IdVec<TMarker, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &&mut IdSlice<TMarker, TValueB>) -> bool {
        self.as_id_slice().eq(*other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&mut IdSlice<TMarker, TValueB>) -> bool {
        self.as_id_slice().ne(*other)
    }
}

impl<TMarker: ?Sized, TValueA, TValueB> PartialEq<IdSlice<TMarker, TValueB>>
    for IdVec<TMarker, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &IdSlice<TMarker, TValueB>) -> bool {
        self.as_id_slice().eq(other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdSlice<TMarker, TValueB>) -> bool {
        self.as_id_slice().ne(other)
    }
}

impl<TMarker: ?Sized, TValueA, TValueB, const N: usize> PartialEq<IdArray<TMarker, TValueB, N>>
    for IdVec<TMarker, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &IdArray<TMarker, TValueB, N>) -> bool {
        self.as_id_slice().eq(other.as_id_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdArray<TMarker, TValueB, N>) -> bool {
        self.as_id_slice().ne(other.as_id_slice())
    }
}

impl<TMarker: ?Sized, TValueA, TValueB> PartialEq<IdVec<TMarker, TValueB>>
    for IdVec<TMarker, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &IdVec<TMarker, TValueB>) -> bool {
        self.as_vec().eq(other.as_vec())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdVec<TMarker, TValueB>) -> bool {
        self.as_vec().ne(other.as_vec())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker: ?Sized, TValue: PartialOrd> PartialOrd for IdVec<TMarker, TValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_vec().partial_cmp(other.as_vec())
    }

    fn lt(&self, other: &Self) -> bool {
        self.as_vec().lt(other.as_vec())
    }

    fn le(&self, other: &Self) -> bool {
        self.as_vec().le(other.as_vec())
    }

    fn gt(&self, other: &Self) -> bool {
        self.as_vec().gt(other.as_vec())
    }

    fn ge(&self, other: &Self) -> bool {
        self.as_vec().ge(other.as_vec())
    }
}
