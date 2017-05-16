# borrow-bag

A type-safe, heterogeneous collection with zero-cost add and borrow.

`BorrowBag` allows the storage of any value, and returns a `Handle` which can be
used to borrow the value back later. As the `BorrowBag` is add-only, `Handle`
values remain valid for the lifetime of the `BorrowBag`.

For usage details, please see the [documentation](https://docs.rs/borrow-bag/)

## Example

```rust
extern crate borrow_bag;

use borrow_bag::new_borrow_bag;

struct X(u8);
struct Y(u8);

fn main() {
    let bag = new_borrow_bag();
    let (bag, x_handle) = bag.add(X(1));
    let (bag, y_handle) = bag.add(Y(2));

    let x: &X = bag.borrow(x_handle);
    assert_eq!(x.0, 1);
    let y: &Y = bag.borrow(y_handle);
    assert_eq!(y.0, 2);
}
```

## License

[MIT](https://opensource.org/licenses/MIT)