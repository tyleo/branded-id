use crate::{IdArray, IdPtr, IdSliceIndex, IdVec, MutIdPtr, UsizeId, internal::fmt_brand_name};
use std::{
    cmp::Ordering,
    fmt,
    fmt::{Arguments, Debug, Formatter},
    hash::{Hash, Hasher},
    io,
    io::{BufRead, IoSlice, IoSliceMut, Read},
    marker::PhantomData,
    mem::transmute,
    net::{SocketAddr, ToSocketAddrs},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

/// A `[TValue]` slice indexed by brand-typed ids ([`UsizeId`] and id ranges)
/// instead of bare `usize`.
#[repr(transparent)]
pub struct IdSlice<TBrand: ?Sized, TValue> {
    phantom: PhantomData<TBrand>,
    repr: [TValue],
}

impl<TBrand: ?Sized, TValue> IdSlice<TBrand, TValue> {
    pub fn as_mut_id_ptr(&mut self) -> MutIdPtr<TBrand, TValue> {
        MutIdPtr::from_mut_ptr(self.as_mut_slice().as_mut_ptr())
    }

    pub fn as_mut_slice(&mut self) -> &mut [TValue] {
        &mut self.repr
    }

    pub const fn as_id_ptr(&self) -> IdPtr<TBrand, TValue> {
        IdPtr::from_ptr(self.as_slice().as_ptr())
    }

    pub const fn as_slice(&self) -> &[TValue] {
        &self.repr
    }

    pub const fn end(&self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.len())
    }

    pub fn from_mut_slice(repr: &mut [TValue]) -> &mut Self {
        unsafe { transmute(repr) }
    }

    pub const fn from_slice(repr: &[TValue]) -> &Self {
        unsafe { transmute(repr) }
    }

    pub fn get<I: IdSliceIndex<Self>>(&self, index: I) -> Option<&I::Output> {
        index.get(self)
    }

    pub fn get_mut<I: IdSliceIndex<Self>>(&mut self, index: I) -> Option<&mut I::Output> {
        index.get_mut(self)
    }

    pub const fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    pub fn iter(&self) -> Iter<'_, TValue> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, TValue> {
        self.as_mut_slice().iter_mut()
    }

    pub const fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<TBrand: ?Sized, TValue> AsMut<IdSlice<TBrand, TValue>> for IdSlice<TBrand, TValue> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<TBrand: ?Sized, TValue> AsRef<IdSlice<TBrand, TValue>> for IdSlice<TBrand, TValue> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<TBrand: ?Sized> BufRead for &IdSlice<TBrand, u8> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        let repr: &mut &[u8] = unsafe { transmute(self) };
        repr.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        let repr: &mut &[u8] = unsafe { transmute(self) };
        repr.consume(amt)
    }

    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
        let repr: &mut &[u8] = unsafe { transmute(self) };
        repr.read_until(byte, buf)
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        let repr: &mut &[u8] = unsafe { transmute(self) };
        repr.read_line(buf)
    }
}

impl<TBrand: ?Sized, TValue: Debug> Debug for IdSlice<TBrand, TValue> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_brand_name::<TBrand>(f)?;
        Debug::fmt(self.as_slice(), f)
    }
}

impl<TBrand: ?Sized, TValue> Default for &IdSlice<TBrand, TValue> {
    fn default() -> Self {
        IdSlice::from_slice(&[])
    }
}

impl<TBrand: ?Sized, TValue> Default for &mut IdSlice<TBrand, TValue> {
    fn default() -> Self {
        IdSlice::from_mut_slice(&mut [])
    }
}

impl<TBrand: ?Sized, TValue> Eq for IdSlice<TBrand, TValue> where [TValue]: PartialEq {}

impl<'a, TBrand: ?Sized, TValue> From<&'a [TValue]> for &'a IdSlice<TBrand, TValue> {
    fn from(value: &'a [TValue]) -> Self {
        IdSlice::from_slice(value)
    }
}

impl<'a, TBrand: ?Sized, TValue> From<&'a mut [TValue]> for &'a IdSlice<TBrand, TValue> {
    fn from(value: &'a mut [TValue]) -> Self {
        IdSlice::from_slice(value)
    }
}

impl<'a, TBrand: ?Sized, TValue> From<&'a mut [TValue]> for &'a mut IdSlice<TBrand, TValue> {
    fn from(value: &'a mut [TValue]) -> Self {
        IdSlice::from_mut_slice(value)
    }
}

impl<TBrand: ?Sized, TValue> Hash for IdSlice<TBrand, TValue>
where
    [TValue]: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<TBrand: ?Sized, TValue, I: IdSliceIndex<IdSlice<TBrand, TValue>>> Index<I>
    for IdSlice<TBrand, TValue>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl<TBrand: ?Sized, TValue, I: IdSliceIndex<IdSlice<TBrand, TValue>>> IndexMut<I>
    for IdSlice<TBrand, TValue>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self)
    }
}

#[allow(clippy::into_iter_on_ref)]
impl<'a, TBrand: ?Sized, TValue> IntoIterator for &'a IdSlice<TBrand, TValue> {
    type Item = <&'a [TValue] as IntoIterator>::Item;
    type IntoIter = <&'a [TValue] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}

#[allow(clippy::into_iter_on_ref)]
impl<'a, TBrand: ?Sized, TValue> IntoIterator for &'a mut IdSlice<TBrand, TValue> {
    type Item = <&'a mut [TValue] as IntoIterator>::Item;
    type IntoIter = <&'a mut [TValue] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        <&'a mut [TValue] as IntoIterator>::into_iter(self.as_mut_slice())
    }
}

impl<TBrand: ?Sized, TValue> Ord for IdSlice<TBrand, TValue>
where
    [TValue]: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<TBrand: ?Sized, TValueA, TValueB> PartialEq<IdSlice<TBrand, TValueB>>
    for IdSlice<TBrand, TValueA>
where
    [TValueA]: PartialEq<[TValueB]>,
{
    fn eq(&self, other: &IdSlice<TBrand, TValueB>) -> bool {
        self.as_slice().eq(other.as_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdSlice<TBrand, TValueB>) -> bool {
        self.as_slice().ne(other.as_slice())
    }
}

impl<TBrand: ?Sized, TValueA, TValueB, const N: usize> PartialEq<IdArray<TBrand, TValueB, N>>
    for IdSlice<TBrand, TValueA>
where
    [TValueA]: PartialEq<[TValueB; N]>,
{
    fn eq(&self, other: &IdArray<TBrand, TValueB, N>) -> bool {
        self.as_slice().eq(other.as_array())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdArray<TBrand, TValueB, N>) -> bool {
        self.as_slice().ne(other.as_array())
    }
}

impl<TBrand: ?Sized, TValueA, TValueB> PartialEq<IdVec<TBrand, TValueB>>
    for IdSlice<TBrand, TValueA>
where
    [TValueA]: PartialEq<Vec<TValueB>>,
{
    fn eq(&self, other: &IdVec<TBrand, TValueB>) -> bool {
        self.as_slice().eq(other.as_vec())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdVec<TBrand, TValueB>) -> bool {
        self.as_slice().ne(other.as_vec())
    }
}

impl<TBrand: ?Sized, TValue> PartialOrd for IdSlice<TBrand, TValue>
where
    [TValue]: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }

    fn lt(&self, other: &Self) -> bool {
        self.as_slice().lt(other.as_slice())
    }

    fn le(&self, other: &Self) -> bool {
        self.as_slice().le(other.as_slice())
    }

    fn gt(&self, other: &Self) -> bool {
        self.as_slice().gt(other.as_slice())
    }

    fn ge(&self, other: &Self) -> bool {
        self.as_slice().ge(other.as_slice())
    }
}

impl<TBrand: ?Sized> Read for &IdSlice<TBrand, u8> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read_vectored(bufs)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read_exact(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read_to_string(buf)
    }
}

impl<TBrand: ?Sized, TValue> ToOwned for IdSlice<TBrand, TValue>
where
    [TValue]: ToOwned<Owned = Vec<TValue>>,
{
    type Owned = IdVec<TBrand, TValue>;

    fn to_owned(&self) -> Self::Owned {
        IdVec::from_vec(self.as_slice().to_owned())
    }

    fn clone_into(&self, target: &mut IdVec<TBrand, TValue>) {
        self.as_slice().clone_into(target.as_mut_vec());
    }
}

impl<'a, TBrand: ?Sized> ToSocketAddrs for &'a IdSlice<TBrand, SocketAddr> {
    type Iter = <&'a [SocketAddr] as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.as_slice().to_socket_addrs()
    }
}

impl<TBrand: ?Sized> io::Write for &mut IdSlice<TBrand, u8> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.write_vectored(bufs)
    }

    fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.write_all(data)
    }

    fn flush(&mut self) -> io::Result<()> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.flush()
    }

    fn write_fmt(&mut self, fmt: Arguments) -> io::Result<()> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.write_fmt(fmt)
    }
}
