//! A type-safe, heterogeneous collection with zero-cost add and borrow.

#![deny(missing_docs)]
#![deny(private_in_public)]

pub mod append;
pub mod handle;
pub mod lookup;

use append::Append;
use handle::Handle;
use lookup::Lookup;

/// Creates a new, empty `BorrowBag`.
pub fn new_borrow_bag() -> BorrowBag<()> {
    BorrowBag { v: () }
}

/// `BorrowBag` allows the storage of any value using `add(T)`, and returns a `Handle` which can be
/// used to borrow the value back later. As the `BorrowBag` is add-only, `Handle` values remain
/// valid for the lifetime of the `BorrowBag`.
///
/// After being added, the `Handle` can be passed to `borrow(Handle)`, which will return a
/// reference to the value.
///
/// ```rust
/// use borrow_bag::new_borrow_bag;
///
/// #[derive(PartialEq, Debug)]
/// struct X;
///
/// #[derive(PartialEq, Debug)]
/// struct Y;
///
/// #[derive(PartialEq, Debug)]
/// struct Z;
///
/// let bag = new_borrow_bag();
/// let (bag, x_handle) = bag.add(X);
/// let (bag, y_handle) = bag.add(Y);
/// let (bag, z_handle) = bag.add(Z);
///
/// let x: &X = bag.borrow(x_handle);
/// assert_eq!(x, &X);
/// let y: &Y = bag.borrow(y_handle);
/// assert_eq!(y, &Y);
/// let z: &Z = bag.borrow(z_handle);
/// assert_eq!(z, &Z);
/// ```
pub struct BorrowBag<V> {
    v: V,
}

impl<V> BorrowBag<V> {
    /// Adds a value to the bag.
    ///
    /// Trait bounds are used to constrain and define the `BorrowBag` implementation, but are not
    /// intended to provide any restrictions on the value being added.
    ///
    /// ```rust
    /// # use borrow_bag::new_borrow_bag;
    /// #
    /// let bag = new_borrow_bag();
    /// // Consumes the empty `bag`, and produces a new `bag` containing the value. The `handle`
    /// // can be used to borrow the value back later.
    /// let (bag, handle) = bag.add(15u8);
    /// ```
    pub fn add<T>
        (self,
         t: T)
         -> (BorrowBag<<V as Append<T>>::Output>, Handle<T, <V as Append<T>>::Navigator>)
        where V: Append<T>
    {
        let (v, handle) = Append::append(self.v, t);
        (BorrowBag { v }, handle)
    }

    /// Borrows a value previously added to the bag.
    ///
    /// ```rust
    /// # use borrow_bag::new_borrow_bag;
    /// #
    /// # let bag = new_borrow_bag();
    /// # let (bag, handle) = bag.add(15u8);
    /// #
    /// let i: &u8 = bag.borrow(handle);
    /// assert_eq!(*i, 15);
    /// ```
    pub fn borrow<T, N>(&self, handle: Handle<T, N>) -> &T
        where V: Lookup<T, N>
    {
        drop(handle); // Otherwise it's unused.
        Lookup::<T, N>::get_from(&self.v)
    }
}
