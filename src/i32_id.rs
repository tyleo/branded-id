use crate::{internal::fmt_marker_name, IsizeId, U32Id, UsizeId};
use std::{
    cmp::Ordering,
    fmt,
    fmt::{
        Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, Write,
    },
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    str::FromStr,
};

pub struct I32Id<TMarker: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: i32,
}

impl<TMarker: ?Sized> I32Id<TMarker> {
    fn fmt_helper(
        self,
        fmt_repr: impl FnOnce(&i32, &mut Formatter) -> fmt::Result,
        f: &mut Formatter,
    ) -> fmt::Result {
        fmt_marker_name::<TMarker>(f)?;
        f.write_char('(')?;
        fmt_repr(&self.to_i32(), f)?;
        f.write_char(')')
    }

    pub const fn from_i32(repr: i32) -> Self {
        Self {
            repr,
            phantom: PhantomData,
        }
    }

    pub const fn to_i32(self) -> i32 {
        self.repr
    }

    pub const fn to_isize_id(self) -> IsizeId<TMarker> {
        IsizeId::from_isize(self.to_i32() as isize)
    }

    pub const fn to_u32_id(self) -> U32Id<TMarker> {
        U32Id::from_u32(self.to_i32() as u32)
    }

    pub const fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_i32() as usize)
    }
}

impl<TMarker: ?Sized> Binary for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Binary::fmt, f)
    }
}

impl<TMarker: ?Sized> Clone for I32Id<TMarker> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker: ?Sized> Copy for I32Id<TMarker> {}

impl<TMarker: ?Sized> Debug for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Debug::fmt, f)
    }
}

impl<TMarker: ?Sized> Display for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Display::fmt, f)
    }
}

impl<TMarker: ?Sized> Eq for I32Id<TMarker> {}

impl<TMarker: ?Sized> From<i32> for I32Id<TMarker> {
    fn from(val: i32) -> Self {
        Self::from_i32(val)
    }
}

impl<TMarker: ?Sized> FromStr for I32Id<TMarker> {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(I32Id::from_i32(i32::from_str(s)?))
    }
}

impl<TMarker: ?Sized> Hash for I32Id<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.to_i32().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute(data) };
        i32::hash_slice(data, state)
    }
}

impl<TMarker: ?Sized> LowerExp for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(LowerExp::fmt, f)
    }
}

impl<TMarker: ?Sized> LowerHex for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(LowerHex::fmt, f)
    }
}

impl<TMarker: ?Sized> Octal for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Octal::fmt, f)
    }
}

impl<TMarker: ?Sized> Ord for I32Id<TMarker> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_i32().cmp(&other.to_i32())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_i32(self.to_i32().max(other.to_i32()))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_i32(self.to_i32().min(other.to_i32()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_i32(self.to_i32().clamp(min.to_i32(), max.to_i32()))
    }
}

impl<TMarker: ?Sized> PartialEq for I32Id<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.to_i32().eq(&other.to_i32())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.to_i32().ne(&other.to_i32())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker: ?Sized> PartialOrd for I32Id<TMarker> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_i32().partial_cmp(&other.to_i32())
    }

    fn lt(&self, other: &Self) -> bool {
        self.to_i32().lt(&other.to_i32())
    }

    fn le(&self, other: &Self) -> bool {
        self.to_i32().le(&other.to_i32())
    }

    fn gt(&self, other: &Self) -> bool {
        self.to_i32().gt(&other.to_i32())
    }

    fn ge(&self, other: &Self) -> bool {
        self.to_i32().ge(&other.to_i32())
    }
}

impl<TMarker: ?Sized> UpperExp for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(UpperExp::fmt, f)
    }
}

impl<TMarker: ?Sized> UpperHex for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(UpperHex::fmt, f)
    }
}
