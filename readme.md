# borrow-bag

A type-safe, heterogeneous collection with zero-cost add and borrow.

`BorrowBag` allows the storage of any value, and returns a `Handle` which can be
used to borrow the value back later. As the `BorrowBag` is add-only, `Handle`
values remain valid for the lifetime of the `BorrowBag`.

For usage details, please see the [documentation](https://docs.rs/borrow-bag/)

## Motivation

Initially, `BorrowBag` was conceived to solve the problem of assembling Gotham's
`Middleware` and `Pipeline` structures, storing concrete types without losing
their type information, and with an ability to borrow them back later after
moving the collection.

The original implementation was generic enough that it was immediately extracted
into this crate.

## Use cases

Please create an issue or pull request so your use case can be added here.

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

    // Type annotations aren't necessary, the `Handle` carries the necessary
    // type information.
    let y = bag.borrow(y_handle);
    assert_eq!(y.0, 2);
}
```

## License

`BorrowBag` is licensed under your option of:

* [MIT License](LICENSE-MIT)
* [Apache License, Version 2.0](LICENSE-APACHE)

## Help
The Gotham core team collaborate on the [#gotham](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23gotham) channel on irc.mozilla.org. Gotham specific chat and requests for help are both very welcome here.

Additionally the Gotham core team often hang out in
[#rust-webdev](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-webdev) and [#rust](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust).

## Policies
Gotham is a young project that we want to create an energetic and respectful community around.

As a starting point we've adopted the following [policies](https://github.com/gotham-rs/policies) which we'd like your help in refining further.

These policies are in effect for any environment or tool that supports the Gotham project.

## News
You can keep up with Gotham at:

* [Our blog](https://staging.gotham.rs/blog)
* [On Twitter](https://twitter.com/gotham_rs)
