use crate::{UsizeId, internal::Sealed};

pub trait UsizeExt: Sealed {
    fn to_usize_id<TBrand: ?Sized>(self) -> UsizeId<TBrand>;
}

impl UsizeExt for usize {
    fn to_usize_id<TBrand: ?Sized>(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self)
    }
}
