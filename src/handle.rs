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

/// Creates a new `Handle` of any given type.
pub fn new_handle<T, N>() -> Handle<T, N> {
    Handle { phantom: PhantomData }
}

impl<T, N> Clone for Handle<T, N> {
    fn clone(&self) -> Handle<T, N> {
        new_handle()
    }
}

// Derived `Copy` doesn't work here.
impl<T, N> Copy for Handle<T, N> {}
