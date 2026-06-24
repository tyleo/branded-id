use crate::{U128Id, internal::fmt_brand_name};
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
};
use uuid::Uuid;

/// A brand-typed UUID id. The `TBrand` type parameter makes ids built for
/// different domains distinct types, so they cannot be mixed even though they
/// share a [`Uuid`] representation.
///
/// Like the string ids, and unlike the integer ids, a `UuidId` is an opaque
/// branded key rather than an index: it does not implement [`Id`](crate::Id) or
/// [`Scalar`](crate::Scalar) and cannot index a container. Convert to and from a
/// raw [`Uuid`] with [`from_uuid`](Self::from_uuid) and [`to_uuid`](Self::to_uuid),
/// or the equivalent `From` impls.
#[repr(transparent)]
pub struct UuidId<TBrand: ?Sized> {
    phantom: PhantomData<TBrand>,
    repr: Uuid,
}

impl<TBrand: ?Sized> UuidId<TBrand> {
    fn fmt_helper(
        self,
        fmt_repr: impl FnOnce(&Uuid, &mut fmt::Formatter) -> fmt::Result,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        use fmt::Write as _;
        fmt_brand_name::<TBrand>(f)?;
        f.write_char('(')?;
        fmt_repr(&self.to_uuid(), f)?;
        f.write_char(')')
    }

    /// Wraps a raw [`Uuid`] as a branded id.
    pub const fn from_uuid(repr: Uuid) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
    }

    /// Returns the underlying [`Uuid`].
    pub const fn to_uuid(self) -> Uuid {
        self.repr
    }

    /// Reinterprets the id as a [`U128Id`] of the same brand, reading the UUID's
    /// 16 bytes as a big-endian `u128`. Lossless and the inverse of
    /// [`U128Id::to_uuid_id`].
    pub const fn to_u128_id(self) -> U128Id<TBrand> {
        U128Id::from_u128(self.to_uuid().as_u128())
    }
}

impl<TBrand: ?Sized> Clone for UuidId<TBrand> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TBrand: ?Sized> Copy for UuidId<TBrand> {}

impl<TBrand: ?Sized> fmt::Debug for UuidId<TBrand> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_helper(fmt::Debug::fmt, f)
    }
}

impl<TBrand: ?Sized> fmt::Display for UuidId<TBrand> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_helper(fmt::Display::fmt, f)
    }
}

impl<TBrand: ?Sized> PartialEq for UuidId<TBrand> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.repr, &other.repr)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        PartialEq::ne(&self.repr, &other.repr)
    }
}

impl<TBrand: ?Sized> Eq for UuidId<TBrand> {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TBrand: ?Sized> PartialOrd for UuidId<TBrand> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.repr, &other.repr)
    }
}

impl<TBrand: ?Sized> Ord for UuidId<TBrand> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.repr, &other.repr)
    }
}

impl<TBrand: ?Sized> Hash for UuidId<TBrand> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Hash::hash(&self.repr, state)
    }
}

impl<TBrand: ?Sized> AsRef<Uuid> for UuidId<TBrand> {
    fn as_ref(&self) -> &Uuid {
        &self.repr
    }
}

impl<TBrand: ?Sized> FromStr for UuidId<TBrand> {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_uuid(<Uuid as FromStr>::from_str(s)?))
    }
}

impl<TBrand: ?Sized> From<Uuid> for UuidId<TBrand> {
    fn from(value: Uuid) -> Self {
        Self::from_uuid(value)
    }
}

impl<TBrand: ?Sized> From<UuidId<TBrand>> for Uuid {
    fn from(value: UuidId<TBrand>) -> Uuid {
        value.to_uuid()
    }
}
