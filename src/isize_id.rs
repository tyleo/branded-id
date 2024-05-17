use crate::{internal::fmt_marker_name, I32Id, U32Id, UsizeId};
use std::{
    cmp::Ordering,
    fmt::{
        self, Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex,
        Write,
    },
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    str::FromStr,
};

pub struct IsizeId<TMarker: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: isize,
}

impl<TMarker: ?Sized> IsizeId<TMarker> {
    fn fmt_helper(
        self,
        fmt_repr: impl FnOnce(&isize, &mut Formatter) -> fmt::Result,
        f: &mut Formatter,
    ) -> fmt::Result {
        fmt_marker_name::<TMarker>(f)?;
        f.write_char('(')?;
        fmt_repr(&self.to_isize(), f)?;
        f.write_char(')')
    }

    pub const fn from_isize(isize: isize) -> IsizeId<TMarker> {
        Self {
            phantom: PhantomData,
            repr: isize,
        }
    }

    pub const fn offset(self, value: isize) -> IsizeId<TMarker> {
        IsizeId::from_isize(self.to_isize() + value)
    }

    pub const fn to_i32_id(self) -> I32Id<TMarker> {
        I32Id::from_i32(self.to_isize() as i32)
    }

    pub const fn to_isize(self) -> isize {
        self.repr
    }

    pub const fn to_u32_id(self) -> U32Id<TMarker> {
        U32Id::from_u32(self.to_isize() as u32)
    }

    pub const fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_isize() as usize)
    }
}

impl<TMarker: ?Sized> Binary for IsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Binary::fmt, f)
    }
}

impl<TMarker: ?Sized> Clone for IsizeId<TMarker> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker: ?Sized> Copy for IsizeId<TMarker> {}

impl<TMarker: ?Sized> Debug for IsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Debug::fmt, f)
    }
}

impl<TMarker: ?Sized> Display for IsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Display::fmt, f)
    }
}

impl<TMarker: ?Sized> Eq for IsizeId<TMarker> {}

impl<TMarker: ?Sized> From<isize> for IsizeId<TMarker> {
    fn from(val: isize) -> Self {
        Self::from_isize(val)
    }
}

impl<TMarker: ?Sized> FromStr for IsizeId<TMarker> {
    type Err = <isize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IsizeId::from_isize(isize::from_str(s)?))
    }
}

impl<TMarker: ?Sized> Hash for IsizeId<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.to_isize().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute(data) };
        isize::hash_slice(data, state)
    }
}

impl<TMarker: ?Sized> LowerExp for IsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(LowerExp::fmt, f)
    }
}

impl<TMarker: ?Sized> LowerHex for IsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(LowerHex::fmt, f)
    }
}

impl<TMarker: ?Sized> Octal for IsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Octal::fmt, f)
    }
}

impl<TMarker: ?Sized> Ord for IsizeId<TMarker> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_isize().cmp(&other.to_isize())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_isize(self.to_isize().max(other.to_isize()))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_isize(self.to_isize().min(other.to_isize()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_isize(self.to_isize().clamp(min.to_isize(), max.to_isize()))
    }
}

impl<TMarker: ?Sized> PartialEq for IsizeId<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.to_isize().eq(&other.to_isize())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.to_isize().ne(&other.to_isize())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker: ?Sized> PartialOrd for IsizeId<TMarker> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_isize().partial_cmp(&other.to_isize())
    }

    fn lt(&self, other: &Self) -> bool {
        self.to_isize().lt(&other.to_isize())
    }

    fn le(&self, other: &Self) -> bool {
        self.to_isize().le(&other.to_isize())
    }

    fn gt(&self, other: &Self) -> bool {
        self.to_isize().gt(&other.to_isize())
    }

    fn ge(&self, other: &Self) -> bool {
        self.to_isize().ge(&other.to_isize())
    }
}

impl<TMarker: ?Sized> UpperExp for IsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(UpperExp::fmt, f)
    }
}

impl<TMarker: ?Sized> UpperHex for IsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(UpperHex::fmt, f)
    }
}
