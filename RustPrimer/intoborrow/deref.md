#Deref

`Deref` is a trait of the `deref` operator `*`, such as `*v`.

It is generally understood that `*v` operation is the reverse operation of `&v`, that is, trying to obtain a copy of a resource from a resource reference (if the resource type implements `Copy`), or ownership (the resource type does not implement `Copy` ).

In Rust, the behavior of this operator can be overloaded. This is also a fundamental feature of Rust operators. Nothing special per se.

## Mandatory implicit conversion (coercion)

The magic of `Deref` is not in the sense of `dereferencing` itself. Rust designers have added a feature to it: `forced implicit conversion`, which is its magic.

The rules for this implicit conversion are:

An object `foo` of type `T`, if `T: Deref<Target=U>`, then a smart pointer or reference (such as `&foo`) related to `foo` will be automatically converted when it is applied into `&U`.

At first glance, this rule seems to be similar to `AsRef`, but it seems to have nothing to do with `dequote`. There are actually some mysteries in it.

When the Rust compiler performs `*v` operations, it will automatically perform reference normalization operations on `v`, that is, convert it into the internal universal reference form `&v`, and the entire expression will become `*&v`. There are two situations here:

1. Convert other types of pointers (such as `Box`, `Rc`, `Arc`, `Cow`, etc. defined in the library) to the internal standard form `&v`;
2. Simplify multiple `&` (for example: `&&&&&&&v`) into `&v` (dequote by inserting enough `*`).

So, it actually does a reference normalization before dereferencing.

Why do you want to turn? Because the ability of the compiler design is that it can only dereference the reference of `&v`. It does not recognize other forms, so it needs to perform reference normalization operations.

Using references for transitions is also to prevent unnecessary copies.

Here are some examples:

```rust
fn foo(s: &str) {
     // borrow a string for a second
}

// String implements Deref<Target=str>
let owned = "Hello".to_string();

// therefore, this works:
foo(&owned);
```

Because `String` implements `Deref<Target=str>`.

```rust
use std::rc::Rc;

fn foo(s: &str) {
     // borrow a string for a second
}

// String implements Deref<Target=str>
let owned = "Hello".to_string();
let counted = Rc::new(owned);

// therefore, this works:
foo(&counted);
```
Because `Rc<T>` implements `Deref<Target=T>`.

```rust
fn foo(s: &[i32]) {
     //borrow a slice for a second
}

// Vec<T> implements Deref<Target=[T]>
let owned = vec![1, 2, 3];

foo(&owned);
```

Because `Vec<T>` implements `Deref<Target=[T]>`.

```rust
struct Foo;

impl Foo {
    fn foo(&self) { println!("Foo"); }
}

let f = &&Foo;

f.foo();
(&f).foo();
(&&f).foo();
(&&&&&&&&f).foo();
```

The above function calls have the same effect.


The design of `coercion` is the only type implicit conversion in Rust. Its purpose is to simplify the writing of programs and make the code less complicated. Freeing people from endless type details makes writing Rust code a joy.
