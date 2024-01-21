use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
};

#[derive(Debug)]
pub struct I32Id<TMarker: ?Sized> {
    repr: i32,
    phantom: PhantomData<TMarker>,
}

impl<TMarker> I32Id<TMarker> {
    pub const fn from_i32(repr: i32) -> Self {
        Self {
            repr,
            phantom: PhantomData,
        }
    }

    pub fn offset(self, value: i32) -> Self {
        (self.repr + value).into()
    }

    pub const fn to_i32(self) -> i32 {
        self.repr
    }
}

impl<TMarker> Clone for I32Id<TMarker> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker> Copy for I32Id<TMarker> {}

impl<TMarker> Eq for I32Id<TMarker> {}

impl<TMarker> From<i32> for I32Id<TMarker> {
    fn from(val: i32) -> Self {
        Self::from_i32(val)
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
