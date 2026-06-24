use crate::{StrId, StringId, extends::Extends};

impl<TBrand: ?Sized> StrId<TBrand> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &StrId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        StrId::from_str(self.as_str())
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &StrId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        StrId::from_str(self.as_str())
    }
}

impl<TBrand: ?Sized> StringId<TBrand> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &StringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        // SAFETY: StringId is #[repr(transparent)] and the brand is a
        // zero-sized PhantomData, so only the brand type parameter changes,
        // not the layout.
        unsafe { ::std::mem::transmute::<&StringId<TBrand>, &StringId<TExtendedBrand>>(self) }
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &StringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        // SAFETY: see downcast_as.
        unsafe { ::std::mem::transmute::<&StringId<TBrand>, &StringId<TExtendedBrand>>(self) }
    }

    /// Mutably reborrows toward the extending brand.
    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut StringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        // SAFETY: see downcast_as.
        unsafe {
            ::std::mem::transmute::<&mut StringId<TBrand>, &mut StringId<TExtendedBrand>>(self)
        }
    }

    /// Mutably reborrows toward the base brand.
    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut StringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        // SAFETY: see downcast_as.
        unsafe {
            ::std::mem::transmute::<&mut StringId<TBrand>, &mut StringId<TExtendedBrand>>(self)
        }
    }

    /// Consumes and rebrands toward the extending brand.
    pub fn downcast_into<TExtendedBrand>(self) -> StringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        StringId::from_string(self.into_string())
    }

    /// Consumes and rebrands toward the base brand.
    pub fn upcast_into<TExtendedBrand>(self) -> StringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        StringId::from_string(self.into_string())
    }
}
