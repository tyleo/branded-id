use crate::{PathBufId, PathId, extends::Extends};

impl<TBrand: ?Sized> PathId<TBrand> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &PathId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        PathId::from_path(self.as_path())
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &PathId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        PathId::from_path(self.as_path())
    }
}

impl<TBrand: ?Sized> PathBufId<TBrand> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &PathBufId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        // SAFETY: PathBufId is #[repr(transparent)] and the brand is a
        // zero-sized PhantomData, so only the brand type parameter changes,
        // not the layout.
        unsafe { ::std::mem::transmute::<&PathBufId<TBrand>, &PathBufId<TExtendedBrand>>(self) }
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &PathBufId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        // SAFETY: see downcast_as.
        unsafe { ::std::mem::transmute::<&PathBufId<TBrand>, &PathBufId<TExtendedBrand>>(self) }
    }

    /// Mutably reborrows toward the extending brand.
    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut PathBufId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        // SAFETY: see downcast_as.
        unsafe {
            ::std::mem::transmute::<&mut PathBufId<TBrand>, &mut PathBufId<TExtendedBrand>>(self)
        }
    }

    /// Mutably reborrows toward the base brand.
    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut PathBufId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        // SAFETY: see downcast_as.
        unsafe {
            ::std::mem::transmute::<&mut PathBufId<TBrand>, &mut PathBufId<TExtendedBrand>>(self)
        }
    }

    /// Consumes and rebrands toward the extending brand.
    pub fn downcast_into<TExtendedBrand>(self) -> PathBufId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        PathBufId::from_path_buf(self.into_path_buf())
    }

    /// Consumes and rebrands toward the base brand.
    pub fn upcast_into<TExtendedBrand>(self) -> PathBufId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        PathBufId::from_path_buf(self.into_path_buf())
    }
}
