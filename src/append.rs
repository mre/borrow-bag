#![doc(hidden)]

use handle::{Handle, Skip, Take};
use lookup::Lookup;

pub trait Append<T> {
    /// The resulting type after adding an element of type `T`.
    type Output: PrefixedWith<Self> + Lookup<T, Self::Navigator>;

    /// A type describing how to borrow the `T` which is added.
    ///
    /// If the output type is `(X, (Y, (Z, ())))`, we've just added the `Z` and so our `Navigator`
    /// will be `(Skip, (Skip, Take))`
    type Navigator;

    /// Append the element, returning a new collection and a handle to borrow the element back.
    fn append(self, t: T) -> (Self::Output, Handle<T, Self::Navigator>);
}

impl<T, U, V> Append<T> for (U, V)
    where V: Append<T>
{
    // We're mid-list. Return the head and append to the tail.
    type Output = (U, <V as Append<T>>::Output);

    // We're mid-list. Skip this element and navigate again in the tail.
    type Navigator = (Skip, <V as Append<T>>::Navigator);

    fn append(self, t: T) -> (Self::Output, Handle<T, Self::Navigator>) {
        let (u, v) = self;
        ((u, v.append(t).0), Handle::new())
    }
}

impl<T> Append<T> for () {
    // This is the end of the added elements. We insert our `T` before the end.
    type Output = (T, ());

    // We're adding our `T` here, so we take this element on navigation.
    type Navigator = Take;

    fn append(self, t: T) -> (Self::Output, Handle<T, Self::Navigator>) {
        ((t, ()), Handle::new())
    }
}

/// Provides proof that the existing list elements don't move, which guarantees that existing
/// `Handle` values continue to work.
pub trait PrefixedWith<T> where T: ?Sized {}

impl<U, V0, V1> PrefixedWith<(U, V0)> for (U, V1) where V1: PrefixedWith<V0> {}
impl<U> PrefixedWith<()> for (U, ()) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_test() {
        let list = ();
        let (list, _): ((u8, ()), Handle<u8, Take>) = list.append(1u8);
        let (list, _) = list.append(2u8);
        let (list, _) = list.append(3u8);

        assert_eq!(list.0, 1);
        assert_eq!((list.1).0, 2);
        assert_eq!(((list.1).1).0, 3);
    }
}
