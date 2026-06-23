use crate::{U32Id, internal::Sealed};

pub trait U32Ext: Sealed {
    fn to_u32_id<TBrand: ?Sized>(self) -> U32Id<TBrand>;
}

impl U32Ext for u32 {
    fn to_u32_id<TBrand: ?Sized>(self) -> U32Id<TBrand> {
        U32Id::from_u32(self)
    }
}
