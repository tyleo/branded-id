/// Definition of errors used in the cross_mem module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CrossMemError {
    /// Memory allocation failed in the separate module.
    AllocFailure,

    /// The requested size overflowed the maximum allowed size.
    SizeOverflow,
}
