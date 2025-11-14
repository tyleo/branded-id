use {
    crate::{LayoutExt, PowerOfTwoUsize},
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
    pub fn from_size_align(size: NonZeroUsize, align: PowerOfTwoUsize) -> Option<Self> {
        if Layout::does_size_rounded_to_multiple_of_align_overflow_isize(size.get(), align) {
            return None;
        }

        let size = size.get();
        let align = align.as_usize();

        let layout = unsafe { Layout::from_size_align_unchecked(size, align) };

        Some(AllocSafeLayout { layout })
    }

    /// Returns the underlying [`Layout`].
    pub const fn as_layout(&self) -> &Layout {
        &self.layout
    }
}
