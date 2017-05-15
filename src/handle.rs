#![doc(hidden)]

use std::marker::PhantomData;

/// Navigator type describing a skipped element
pub struct Skip;

/// Navigator type describing the target element
pub struct Take;

/// A zero-sized wrapped for the navigator. Returned from `BorrowBag::add`, and used to borrow back
/// the element which was added.
pub struct Handle<T, N> {
    phantom: PhantomData<(T, N)>,
}

impl<T, N> Handle<T, N> {
    /// Creates a new `Handle` of any given type.
    pub fn new() -> Handle<T, N> {
        Handle { phantom: PhantomData }
    }
}
