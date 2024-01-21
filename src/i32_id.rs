use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
};

#[derive(Debug)]
pub struct I32Id<TMarker: ?Sized> {
    i32: i32,
    phantom: PhantomData<TMarker>,
}

impl<TMarker> I32Id<TMarker> {
    pub const fn from_i32(i32: i32) -> I32Id<TMarker> {
        I32Id {
            i32,
            phantom: PhantomData,
        }
    }

    pub fn offset(self, value: i32) -> I32Id<TMarker> {
        (self.i32 + value).into()
    }

    pub const fn to_i32(self) -> i32 {
        self.i32
    }
}

impl<TMarker> Clone for I32Id<TMarker> {
    fn clone(&self) -> I32Id<TMarker> {
        *self
    }
}

impl<TMarker> Copy for I32Id<TMarker> {}

impl<TMarker> Eq for I32Id<TMarker> {}

impl<TMarker> From<i32> for I32Id<TMarker> {
    fn from(val: i32) -> Self {
        I32Id::from_i32(val)
    }
}

impl<TMarker> Hash for I32Id<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.i32.hash(state)
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
        self.i32.cmp(&other.i32)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.i32.max(other.i32).into()
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.i32.min(other.i32).into()
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        self.i32.clamp(min.i32, max.i32).into()
    }
}

impl<TMarker> PartialEq for I32Id<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.i32 == other.i32
    }
}

impl<TMarker> PartialOrd for I32Id<TMarker> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        self.i32.lt(&other.i32)
    }

    fn le(&self, other: &Self) -> bool {
        self.i32.le(&other.i32)
    }

    fn gt(&self, other: &Self) -> bool {
        self.i32.gt(&other.i32)
    }

    fn ge(&self, other: &Self) -> bool {
        self.i32.ge(&other.i32)
    }
}
