use crate::{EnumerateIds, Id};

/// Id-typed adapters for every iterator.
pub trait IteratorExt: Iterator + Sized {
    /// Pairs each item with an id counting from zero the way
    /// [`enumerate`](Iterator::enumerate) pairs it with a position.
    ///
    /// # Panics
    /// The adapter panics on reaching a position the id width cannot hold.
    fn enumerate_ids<TId: Id>(self) -> EnumerateIds<Self, TId> {
        EnumerateIds::new(self)
    }
}

impl<TIter: Iterator> IteratorExt for TIter {}
