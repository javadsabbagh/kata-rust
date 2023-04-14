# Borrow, BorrowMut, ToOwned

## Borrow<T>

`use std::borrow::Borrow;`

`Borrow` provides a method `.borrow()`.

For a value `foo` of type `T`, if `T` implements `Borrow<U>`, then `foo` can execute the `.borrow()` operation, namely `foo.borrow()`. As a result of the operation, we get a new reference of type `&U`.

`Borrow` can be thought of as a strict version of `AsRef`, which imposes some additional restrictions on the types before and after the universal reference operation.

There must be internal equivalence between the preceding and following types of `Borrow`. `Borrow` cannot be implemented between two types that do not have this equivalence.

`AsRef` is more generic, more general, covers more types, and is a superset of `Borrow`.

Example:

```rust
use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
     assert_eq!("Hello", s.borrow());
}

let s = "Hello".to_string();

check(s);

let s = "Hello";

check(s);
```

## BorrowMut<T>

`use std::borrow::BorrowMut;`

`BorrowMut<T>` provides a method `.borrow_mut()`. It is a mutable reference version of `Borrow<T>`.

For a value `foo` of type `T`, if `T` implements `BorrowMut<U>`, then `foo` can execute the `.borrow_mut()` operation, namely `foo.borrow_mut()`. As a result of the operation we get a mutable reference of type `&mut U`.

Note: During conversion, `foo` will be borrowed by mutable.

##ToOwned

`use std::borrow::ToOwned;`

`ToOwned` is the generic version of `Clone`. It provides `.to_owned()` method for type conversion.

Some type `T` that implements `Clone` can generate an instance of `T` with ownership from the reference state instance `&T` through the `.clone()` method. But it can only generate `T` from `&T`. For other forms of references, `Clone` can't do anything.

The `ToOwned` trait is capable of generating an owned type instance from any reference type instance.

## refer to

- [http://doc.rust-lang.org/std/borrow/trait.Borrow.html](http://doc.rust-lang.org/std/borrow/trait.Borrow.html)
