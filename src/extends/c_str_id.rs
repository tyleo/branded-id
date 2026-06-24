use crate::{CStrId, CStringId, extends::Extends};

impl<TBrand: ?Sized> CStrId<TBrand> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &CStrId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        CStrId::from_c_str(self.as_c_str())
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &CStrId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        CStrId::from_c_str(self.as_c_str())
    }
}

impl<TBrand: ?Sized> CStringId<TBrand> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &CStringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        // SAFETY: CStringId is #[repr(transparent)] and the brand is a
        // zero-sized PhantomData, so only the brand type parameter changes,
        // not the layout.
        unsafe { ::std::mem::transmute::<&CStringId<TBrand>, &CStringId<TExtendedBrand>>(self) }
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &CStringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        // SAFETY: see downcast_as.
        unsafe { ::std::mem::transmute::<&CStringId<TBrand>, &CStringId<TExtendedBrand>>(self) }
    }

    /// Mutably reborrows toward the extending brand.
    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut CStringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        // SAFETY: see downcast_as.
        unsafe {
            ::std::mem::transmute::<&mut CStringId<TBrand>, &mut CStringId<TExtendedBrand>>(self)
        }
    }

    /// Mutably reborrows toward the base brand.
    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut CStringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        // SAFETY: see downcast_as.
        unsafe {
            ::std::mem::transmute::<&mut CStringId<TBrand>, &mut CStringId<TExtendedBrand>>(self)
        }
    }

    /// Consumes and rebrands toward the extending brand.
    pub fn downcast_into<TExtendedBrand>(self) -> CStringId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        CStringId::from_c_string(self.into_c_string())
    }

    /// Consumes and rebrands toward the base brand.
    pub fn upcast_into<TExtendedBrand>(self) -> CStringId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        CStringId::from_c_string(self.into_c_string())
    }
}
