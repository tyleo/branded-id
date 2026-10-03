use std::fmt;

/// Prints a slice of ids as `[1, 2]`, the way `{:?}` prints a slice of their
/// integers. Formatter flags apply to each id.
/// [`SliceExt::display_ids`](crate::SliceExt::display_ids) builds one.
pub struct DisplayIds<'a, TId> {
    ids: &'a [TId],
}

impl<'a, TId> DisplayIds<'a, TId> {
    pub(crate) fn new(ids: &'a [TId]) -> Self {
        Self { ids }
    }
}

impl<TId: fmt::Debug> fmt::Debug for DisplayIds<'_, TId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.ids, f)
    }
}

impl<TId: fmt::Display> fmt::Display for DisplayIds<'_, TId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[")?;

        for (index, id) in self.ids.iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }

            fmt::Display::fmt(id, f)?;
        }

        f.write_str("]")
    }
}
