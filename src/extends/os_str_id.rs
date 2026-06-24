use crate::{OsStrId, OsStringId, extends::Extends};

impl<TBrand: ?Sized> OsStrId<TBrand> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &OsStrId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        OsStrId::from_os_str(self.as_os_str())
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &OsStrId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        OsStrId::from_os_str(self.as_os_str())
    }
}

impl<TBrand: ?Sized> OsStringId<TBrand> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &OsStringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        // SAFETY: OsStringId is #[repr(transparent)] and the brand is a
        // zero-sized PhantomData, so only the brand type parameter changes,
        // not the layout.
        unsafe { ::std::mem::transmute::<&OsStringId<TBrand>, &OsStringId<TExtendedBrand>>(self) }
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &OsStringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        // SAFETY: see downcast_as.
        unsafe { ::std::mem::transmute::<&OsStringId<TBrand>, &OsStringId<TExtendedBrand>>(self) }
    }

    /// Mutably reborrows toward the extending brand.
    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut OsStringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        // SAFETY: see downcast_as.
        unsafe {
            ::std::mem::transmute::<&mut OsStringId<TBrand>, &mut OsStringId<TExtendedBrand>>(self)
        }
    }

    /// Mutably reborrows toward the base brand.
    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut OsStringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        // SAFETY: see downcast_as.
        unsafe {
            ::std::mem::transmute::<&mut OsStringId<TBrand>, &mut OsStringId<TExtendedBrand>>(self)
        }
    }

    /// Consumes and rebrands toward the extending brand.
    pub fn downcast_into<TExtendedBrand>(self) -> OsStringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        OsStringId::from_os_string(self.into_os_string())
    }

    /// Consumes and rebrands toward the base brand.
    pub fn upcast_into<TExtendedBrand>(self) -> OsStringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        OsStringId::from_os_string(self.into_os_string())
    }
}
