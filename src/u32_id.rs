use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Add,
};

#[derive(Debug)]
pub struct U32Id<TMarker: ?Sized> {
    u32: u32,
    phantom: PhantomData<TMarker>,
}

impl<TMarker> U32Id<TMarker> {
    pub const fn from_u32(u32: u32) -> U32Id<TMarker> {
        U32Id {
            u32,
            phantom: PhantomData,
        }
    }

    pub const fn to_u32(self) -> u32 {
        self.u32
    }
}

impl<TMarker> Add<u32> for U32Id<TMarker> {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        (self.u32 + rhs).into()
    }
}

impl<TMarker> Add<u32> for &U32Id<TMarker> {
    type Output = U32Id<TMarker>;

    fn add(self, rhs: u32) -> Self::Output {
        (self.u32 + rhs).into()
    }
}

impl<TMarker> Clone for U32Id<TMarker> {
    fn clone(&self) -> U32Id<TMarker> {
        *self
    }
}

impl<TMarker> Copy for U32Id<TMarker> {}

impl<TMarker> Eq for U32Id<TMarker> {}

impl<TMarker> From<u32> for U32Id<TMarker> {
    fn from(val: u32) -> Self {
        U32Id::from_u32(val)
    }
}

impl<TMarker> Hash for U32Id<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.u32.hash(state)
    }
}

impl<TMarker> Ord for U32Id<TMarker> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.u32.cmp(&other.u32)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.u32.max(other.u32).into()
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.u32.min(other.u32).into()
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        self.u32.clamp(min.u32, max.u32).into()
    }
}

impl<TMarker> PartialEq for U32Id<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.u32 == other.u32
    }
}

impl<TMarker> PartialOrd for U32Id<TMarker> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        self.u32.lt(&other.u32)
    }

    fn le(&self, other: &Self) -> bool {
        self.u32.le(&other.u32)
    }

    fn gt(&self, other: &Self) -> bool {
        self.u32.gt(&other.u32)
    }

    fn ge(&self, other: &Self) -> bool {
        self.u32.ge(&other.u32)
    }
}
