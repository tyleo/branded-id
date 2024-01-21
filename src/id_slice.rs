use crate::{IdPtr, IdSliceIndex, IdVec, MutIdPtr, UsizeId};
use std::{
    cmp::Ordering,
    fmt::Arguments,
    hash::{Hash, Hasher},
    io::{BufRead, IoSlice, IoSliceMut, Read, Write},
    marker::PhantomData,
    mem::transmute,
    net::{SocketAddr, ToSocketAddrs},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

#[derive(Debug)]
pub struct IdSlice<TMarker: ?Sized, TValue> {
    phantom: PhantomData<TMarker>,
    repr: [TValue],
}

impl<TMarker, TValue> IdSlice<TMarker, TValue> {
    pub fn from_mut_slice(repr: &mut [TValue]) -> &mut Self {
        unsafe { transmute(repr) }
    }

    pub const fn from_slice(repr: &[TValue]) -> &Self {
        unsafe { transmute(repr) }
    }

    pub fn as_mut_id_ptr(&mut self) -> MutIdPtr<TMarker, TValue> {
        MutIdPtr::from_mut_ptr(self.as_mut_slice().as_mut_ptr())
    }

    pub fn as_mut_slice(&mut self) -> &mut [TValue] {
        &mut self.repr
    }

    pub fn as_id_ptr(&self) -> IdPtr<TMarker, TValue> {
        IdPtr::from_ptr(self.as_slice().as_ptr())
    }

    pub const fn as_slice(&self) -> &[TValue] {
        &self.repr
    }

    pub const fn end(&self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.len())
    }

    pub const fn is_empty(&self) -> bool {
        self.repr.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, TValue> {
        self.repr.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, TValue> {
        self.repr.iter_mut()
    }

    pub const fn len(&self) -> usize {
        self.repr.len()
    }
}

impl<TMarker, TValue> AsMut<IdSlice<TMarker, TValue>> for IdSlice<TMarker, TValue> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<TMarker, TValue> AsRef<IdSlice<TMarker, TValue>> for IdSlice<TMarker, TValue> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<TMarker> BufRead for &IdSlice<TMarker, u8> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        let repr: &mut &[u8] = unsafe { transmute(self) };
        repr.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        let repr: &mut &[u8] = unsafe { transmute(self) };
        repr.consume(amt)
    }

    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        let repr: &mut &[u8] = unsafe { transmute(self) };
        repr.read_until(byte, buf)
    }

    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let repr: &mut &[u8] = unsafe { transmute(self) };
        repr.read_line(buf)
    }
}

impl<TMarker, TValue> Default for &IdSlice<TMarker, TValue> {
    fn default() -> Self {
        IdSlice::from_slice(&[])
    }
}

impl<TMarker, TValue> Default for &mut IdSlice<TMarker, TValue> {
    fn default() -> Self {
        IdSlice::from_mut_slice(&mut [])
    }
}

impl<TMarker, TValue> Eq for IdSlice<TMarker, TValue> where [TValue]: PartialEq {}

impl<'a, TMarker, TValue> From<&'a [TValue]> for &'a IdSlice<TMarker, TValue> {
    fn from(value: &'a [TValue]) -> Self {
        IdSlice::from_slice(value)
    }
}

impl<'a, TMarker, TValue> From<&'a mut [TValue]> for &'a IdSlice<TMarker, TValue> {
    fn from(value: &'a mut [TValue]) -> Self {
        IdSlice::from_slice(value)
    }
}

impl<'a, TMarker, TValue> From<&'a mut [TValue]> for &'a mut IdSlice<TMarker, TValue> {
    fn from(value: &'a mut [TValue]) -> Self {
        IdSlice::from_mut_slice(value)
    }
}

impl<TMarker, TValue> Hash for IdSlice<TMarker, TValue>
where
    [TValue]: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.repr.hash(state)
    }
}

impl<TMarker, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> Index<I>
    for IdSlice<TMarker, TValue>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl<TMarker, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> IndexMut<I>
    for IdSlice<TMarker, TValue>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self)
    }
}

#[allow(clippy::into_iter_on_ref)]
impl<'a, TMarker, TValue> IntoIterator for &'a IdSlice<TMarker, TValue> {
    type Item = <&'a [TValue] as IntoIterator>::Item;
    type IntoIter = <&'a [TValue] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.repr.into_iter()
    }
}

#[allow(clippy::into_iter_on_ref)]
impl<'a, TMarker, TValue> IntoIterator for &'a mut IdSlice<TMarker, TValue> {
    type Item = <&'a mut [TValue] as IntoIterator>::Item;
    type IntoIter = <&'a mut [TValue] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        <&'a mut [TValue] as IntoIterator>::into_iter(&mut self.repr)
    }
}

impl<TMarker, TValue> Ord for IdSlice<TMarker, TValue>
where
    [TValue]: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.repr.cmp(&other.repr)
    }
}

impl<TMarker, TValueA, TValueB> PartialEq<IdSlice<TMarker, TValueB>> for IdSlice<TMarker, TValueA>
where
    [TValueA]: PartialEq<[TValueB]>,
{
    fn eq(&self, other: &IdSlice<TMarker, TValueB>) -> bool {
        self.repr.eq(&other.repr)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdSlice<TMarker, TValueB>) -> bool {
        self.repr.ne(&other.repr)
    }
}

impl<TMarker, TValue> PartialOrd for IdSlice<TMarker, TValue>
where
    [TValue]: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.repr.partial_cmp(&other.repr)
    }

    fn lt(&self, other: &Self) -> bool {
        self.repr.lt(&other.repr)
    }

    fn le(&self, other: &Self) -> bool {
        self.repr.le(&other.repr)
    }

    fn gt(&self, other: &Self) -> bool {
        self.repr.gt(&other.repr)
    }

    fn ge(&self, other: &Self) -> bool {
        self.repr.ge(&other.repr)
    }
}

impl<TMarker> Read for &IdSlice<TMarker, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> std::io::Result<usize> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read_vectored(bufs)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read_exact(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let this: &mut &[u8] = unsafe { transmute(self) };
        this.read_to_string(buf)
    }
}

impl<TMarker, TValue> ToOwned for IdSlice<TMarker, TValue>
where
    TValue: Clone,
{
    type Owned = IdVec<TMarker, TValue>;

    fn to_owned(&self) -> Self::Owned {
        IdVec::from_vec(self.as_slice().to_owned())
    }

    fn clone_into(&self, target: &mut IdVec<TMarker, TValue>) {
        self.repr.clone_into(&mut target.repr);
    }
}

impl<'a, TMarker> ToSocketAddrs for &'a IdSlice<TMarker, SocketAddr> {
    type Iter = <&'a [SocketAddr] as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        self.as_slice().to_socket_addrs()
    }
}

impl<'a, TMarker> Write for &'a mut IdSlice<TMarker, u8> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.write_vectored(bufs)
    }

    fn write_all(&mut self, data: &[u8]) -> std::io::Result<()> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.write_all(data)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.flush()
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        let this: &mut &mut [u8] = unsafe { transmute(self) };
        this.write_fmt(fmt)
    }
}
