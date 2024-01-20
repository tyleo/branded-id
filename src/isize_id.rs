use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct IsizeId<TMarker: ?Sized> {
    isize: isize,
    phantom: PhantomData<TMarker>,
}

impl<TMarker> IsizeId<TMarker> {
    pub const fn from_isize(isize: isize) -> IsizeId<TMarker> {
        IsizeId {
            isize,
            phantom: PhantomData,
        }
    }

    pub const fn offset(self, value: isize) -> IsizeId<TMarker> {
        IsizeId::from_isize(self.to_isize() + value)
    }

    pub const fn to_isize(self) -> isize {
        self.isize
    }
}

impl<TMarker> Clone for IsizeId<TMarker> {
    fn clone(&self) -> IsizeId<TMarker> {
        *self
    }
}

impl<TMarker> Copy for IsizeId<TMarker> {}

impl<TMarker> Eq for IsizeId<TMarker> {}

impl<TMarker> From<isize> for IsizeId<TMarker> {
    fn from(val: isize) -> Self {
        IsizeId::from_isize(val)
    }
}

impl<TMarker> Hash for IsizeId<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.isize.hash(state)
    }
}

impl<TMarker> Ord for IsizeId<TMarker> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.isize.cmp(&other.isize)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.isize.max(other.isize).into()
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.isize.min(other.isize).into()
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        self.isize.clamp(min.isize, max.isize).into()
    }
}

impl<TMarker> PartialEq for IsizeId<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.isize == other.isize
    }
}

impl<TMarker> PartialOrd for IsizeId<TMarker> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        self.isize.lt(&other.isize)
    }

    fn le(&self, other: &Self) -> bool {
        self.isize.le(&other.isize)
    }

    fn gt(&self, other: &Self) -> bool {
        self.isize.gt(&other.isize)
    }

    fn ge(&self, other: &Self) -> bool {
        self.isize.ge(&other.isize)
    }
}
