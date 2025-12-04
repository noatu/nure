# Sorted Linked List

> [!NOTE]
> This one implements tests, so `cargo test`

This code implements a **generic**, sorted Singly Linked List that relies on Rust's `Rc` and `RefCell` smart pointers for shared ownership and interior mutability. Unlike a standard push-append list, the `add` method automatically inserts new elements into their correct sorted position based on their value, ensuring the list remains ordered at all times. It is fully unit-tested and supports operations like removing specific values, popping by index, and deep equality checks.

Test results:
```sh
running 8 tests
test tests::add ... ok
test tests::custom_type ... ok
test tests::equal ... ok
test tests::get ... ok
test tests::pop ... ok
test tests::print ... ok
test tests::remove ... ok
test tests::remove_all ... ok
```
