use {
    crate::PowerOfTwoUsize,
    std::{alloc::Layout, num::NonZeroUsize},
};

/// A layout that is guaranteed to be safe for allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AllocSafeLayout {
    layout: Layout,
}

impl AllocSafeLayout {
    /// Creates a new [`AllocSafeLayout`] from size and alignment.
    pub const fn from_size_align(size: NonZeroUsize, align: PowerOfTwoUsize) -> Self {
        let size = size.get();
        let align = align.get_usize();

        let layout = unsafe { Layout::from_size_align_unchecked(size, align) };

        AllocSafeLayout { layout }
    }

    /// Returns the underlying [`Layout`].
    pub const fn as_layout(&self) -> &Layout {
        &self.layout
    }
}
