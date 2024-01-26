use crate::{internal::fmt_marker_name, I32Id, IsizeId, UsizeId};
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

pub struct U32Id<TMarker: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: u32,
}

impl<TMarker> U32Id<TMarker> {
    fn fmt_helper(
        self,
        fmt_repr: impl FnOnce(&u32, &mut Formatter) -> fmt::Result,
        f: &mut Formatter,
    ) -> fmt::Result {
        fmt_marker_name::<TMarker>(f)?;
        f.write_char('(')?;
        fmt_repr(&self.to_u32(), f)?;
        f.write_char(')')
    }

    pub const fn from_u32(repr: u32) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
    }

    pub const fn to_i32_id(self) -> I32Id<TMarker> {
        I32Id::from_i32(self.to_u32() as i32)
    }

    pub const fn to_isize_id(self) -> IsizeId<TMarker> {
        IsizeId::from_isize(self.to_u32() as isize)
    }

    pub const fn to_u32(self) -> u32 {
        self.repr
    }

    pub const fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_u32() as usize)
    }
}

impl<TMarker> Binary for U32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Binary::fmt, f)
    }
}

impl<TMarker> Clone for U32Id<TMarker> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker> Copy for U32Id<TMarker> {}

impl<TMarker> Debug for U32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Debug::fmt, f)
    }
}

impl<TMarker> Display for U32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Display::fmt, f)
    }
}

impl<TMarker> Eq for U32Id<TMarker> {}

impl<TMarker> From<u32> for U32Id<TMarker> {
    fn from(val: u32) -> Self {
        Self::from_u32(val)
    }
}

impl<TMarker> FromStr for U32Id<TMarker> {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(U32Id::from_u32(u32::from_str(s)?))
    }
}

impl<TMarker> Hash for U32Id<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.to_u32().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute(data) };
        u32::hash_slice(data, state)
    }
}

impl<TMarker> LowerExp for U32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(LowerExp::fmt, f)
    }
}

impl<TMarker> LowerHex for U32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(LowerHex::fmt, f)
    }
}

impl<TMarker> Octal for U32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Octal::fmt, f)
    }
}

impl<TMarker> Ord for U32Id<TMarker> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_u32().cmp(&other.to_u32())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_u32(self.to_u32().max(other.to_u32()))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_u32(self.to_u32().min(other.to_u32()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_u32(self.to_u32().clamp(min.to_u32(), max.to_u32()))
    }
}

impl<TMarker> PartialEq for U32Id<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.to_u32().eq(&other.to_u32())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.to_u32().ne(&other.to_u32())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker> PartialOrd for U32Id<TMarker> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_u32().partial_cmp(&other.to_u32())
    }

    fn lt(&self, other: &Self) -> bool {
        self.to_u32().lt(&other.to_u32())
    }

    fn le(&self, other: &Self) -> bool {
        self.to_u32().le(&other.to_u32())
    }

    fn gt(&self, other: &Self) -> bool {
        self.to_u32().gt(&other.to_u32())
    }

    fn ge(&self, other: &Self) -> bool {
        self.to_u32().ge(&other.to_u32())
    }
}

impl<TMarker> UpperExp for U32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(UpperExp::fmt, f)
    }
}

impl<TMarker> UpperHex for U32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(UpperHex::fmt, f)
    }
}
