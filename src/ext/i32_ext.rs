use crate::{I32Id, internal::Sealed};

pub trait I32Ext: Sealed {
    fn to_i32_id<TBrand: ?Sized>(self) -> I32Id<TBrand>;
}

impl I32Ext for i32 {
    fn to_i32_id<TBrand: ?Sized>(self) -> I32Id<TBrand> {
        I32Id::from_i32(self)
    }
}
