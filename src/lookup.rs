#![doc(hidden)]

use handle::{Skip, Take};

/// Allows borrowing a value of type `T` from the implementing type.
pub trait Lookup<T, N> {
    /// Borrows the value of type `T`.
    fn get_from(&self) -> &T;
}

impl<T, U, V, N> Lookup<T, (Skip, N)> for (U, V)
    where V: Lookup<T, N>
{
    fn get_from(&self) -> &T {
        self.1.get_from()
    }
}

impl<T, V> Lookup<T, Take> for (T, V) {
    fn get_from(&self) -> &T {
        &self.0
    }
}
