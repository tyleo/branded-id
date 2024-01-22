use crate::{internal::unqualified_type_name, IsizeId, U32Id, UsizeId};
use core::fmt;
use std::{
    any::type_name,
    cmp::Ordering,
    fmt::{
        Alignment, Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp,
        UpperHex, Write,
    },
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    str::FromStr,
};

pub struct I32Id<TMarker: ?Sized> {
    repr: i32,
    phantom: PhantomData<TMarker>,
}

impl<TMarker> I32Id<TMarker> {
    fn fmt_helper(
        self,
        f: &mut Formatter,
        prefix: impl FnOnce(&mut String) -> fmt::Result,
        integer: impl FnOnce(i32, &mut String, &Formatter) -> fmt::Result,
        suffix: impl FnOnce(&mut String) -> fmt::Result,
    ) -> std::fmt::Result {
        let mut str = String::new();

        prefix(&mut str)?;

        if f.alternate() {
            write!(str, "I32Id({}; ", type_name::<TMarker>())?;
        } else {
            write!(str, "I32Id({}; ", unqualified_type_name::<TMarker>())?;
        };

        integer(self.to_i32(), &mut str, f)?;

        write!(str, ")")?;

        suffix(&mut str)?;

        if let Some(width) = f.width() {
            match f.align() {
                Some(Alignment::Center) => write!(f, "{:^width$}", str),
                Some(Alignment::Right) => write!(f, "{:>width$}", str),
                _ => write!(f, "{:<width$}", str),
            }
        } else {
            write!(f, "{}", str)
        }
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

impl<TMarker> Binary for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(
            f,
            |_| Ok(()),
            |i32, str, f| {
                if f.sign_plus() {
                    write!(str, "{:+#b}", i32)
                } else if f.sign_minus() {
                    write!(str, "{:-#b}", i32)
                } else {
                    write!(str, "{:#b}", i32)
                }
            },
            |_| Ok(()),
        )
    }
}

impl<TMarker> Clone for I32Id<TMarker> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker> Copy for I32Id<TMarker> {}

impl<TMarker> Debug for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(
            f,
            |str| write!(str, "\""),
            |i32, str, f| {
                if f.sign_plus() {
                    write!(str, "{:+?}", i32)
                } else if f.sign_minus() {
                    write!(str, "{:-?}", i32)
                } else {
                    write!(str, "{:?}", i32)
                }
            },
            |str| write!(str, "\""),
        )
    }
}

impl<TMarker> Display for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(
            f,
            |_| Ok(()),
            |i32, str, f| {
                if f.sign_plus() {
                    write!(str, "{:+}", i32)
                } else if f.sign_minus() {
                    write!(str, "{:-}", i32)
                } else {
                    write!(str, "{}", i32)
                }
            },
            |_| Ok(()),
        )
    }
}

impl<TMarker> Eq for I32Id<TMarker> {}

impl<TMarker> From<i32> for I32Id<TMarker> {
    fn from(val: i32) -> Self {
        Self::from_i32(val)
    }
}

impl<TMarker> FromStr for I32Id<TMarker> {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(I32Id::from_i32(i32::from_str(s)?))
    }
}

impl<TMarker> Hash for I32Id<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.repr.hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute(data) };
        i32::hash_slice(data, state)
    }
}

impl<TMarker> LowerExp for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(
            f,
            |_| Ok(()),
            |i32, str, f| {
                if f.sign_plus() {
                    write!(str, "{:+#e}", i32)
                } else if f.sign_minus() {
                    write!(str, "{:-#e}", i32)
                } else {
                    write!(str, "{:#e}", i32)
                }
            },
            |_| Ok(()),
        )
    }
}

impl<TMarker> LowerHex for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(
            f,
            |_| Ok(()),
            |i32, str, f| {
                if f.sign_plus() {
                    write!(str, "{:+#x}", i32)
                } else if f.sign_minus() {
                    write!(str, "{:-#x}", i32)
                } else {
                    write!(str, "{:#x}", i32)
                }
            },
            |_| Ok(()),
        )
    }
}

impl<TMarker> Octal for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(
            f,
            |_| Ok(()),
            |i32, str, f| {
                if f.sign_plus() {
                    write!(str, "{:+#o}", i32)
                } else if f.sign_minus() {
                    write!(str, "{:-#o}", i32)
                } else {
                    write!(str, "{:#o}", i32)
                }
            },
            |_| Ok(()),
        )
    }
}

impl<TMarker> Ord for I32Id<TMarker> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.repr.cmp(&other.repr)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_i32(self.repr.max(other.repr))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_i32(self.repr.min(other.repr))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_i32(self.repr.clamp(min.repr, max.repr))
    }
}

impl<TMarker> PartialEq for I32Id<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.repr.eq(&other.repr)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.repr.ne(&other.repr)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker> PartialOrd for I32Id<TMarker> {
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

impl<TMarker> UpperExp for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(
            f,
            |_| Ok(()),
            |i32, str, f| {
                if f.sign_plus() {
                    write!(str, "{:+#E}", i32)
                } else if f.sign_minus() {
                    write!(str, "{:-#E}", i32)
                } else {
                    write!(str, "{:#E}", i32)
                }
            },
            |_| Ok(()),
        )
    }
}

impl<TMarker> UpperHex for I32Id<TMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(
            f,
            |_| Ok(()),
            |i32, str, f| {
                if f.sign_plus() {
                    write!(str, "{:+#X}", i32)
                } else if f.sign_minus() {
                    write!(str, "{:-#X}", i32)
                } else {
                    write!(str, "{:#X}", i32)
                }
            },
            |_| Ok(()),
        )
    }
}
