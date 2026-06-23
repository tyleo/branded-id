use crate::{IsizeId, internal::Sealed};

pub trait IsizeExt: Sealed {
    fn to_isize_id<TBrand: ?Sized>(self) -> IsizeId<TBrand>;
}

impl IsizeExt for isize {
    fn to_isize_id<TBrand: ?Sized>(self) -> IsizeId<TBrand> {
        IsizeId::from_isize(self)
    }
}
