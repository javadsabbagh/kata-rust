# 10.1 trait keyword

## traits and concrete types

Use **trait** to define a trait:

```rust
trait HasArea {
    fn area(&self) -> f64;
}
```

The functions in **trait** can have no function body, and the implementation code is handed over to the type that implements it to supplement:

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };
    println!("circle c has an area of {}", c.area());
}
```

**Note**: **&self** means **area** This function will take the caller's reference as a parameter

This program will output:

```
circle c has an area of 3.141592653589793
```

## traits and generics

> We have learned about the definition and use of trait in Rust, and then we will introduce its usage scenarios, from which we can see the surprises brought by the feature of interface

We know that generics can refer to any type, but sometimes this is not what we want and we need to give it some constraints.

#### Generic trait constraints

```rust
use std::fmt::Debug;
fn foo<T: Debug>(s: T) {
    println!("{:?}", s);
}
```

`Debug` is a built-in trait of **Rust**, which implements printing content for "{:?}". The function `foo` accepts a generic type as a parameter, and agrees that it needs to implement `Debug`

#### Multiple trait constraints

Generics can be constrained using several traits:

```rust
use std::fmt::Debug;
fn foo<T: Debug + Clone>(s: T) {
    s.clone();
    println!("{:?}", s);
}
```

In `<T: Debug + Clone>`, `Debug` and `Clone` are connected by `+`, indicating that the generic type `T` needs to implement these two traits at the same time.

#### where keyword

After the constraint traits are added, the code looks weird. At this time, you need to use the `where` clause:

```rust
use std::fmt::Debug;
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// where clause
fn foo<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// or
fn foo<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}
```

## traits and built-in types

Built-in types such as `i32`, `i64`, etc. can also add trait implementations to customize some functions:

```rust
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        *self as f64
    }
}

5.area();
```

There are limits to such an approach. Rust has an "orphan rule": when you implement a trait for a certain type, at least one of the type or trait must be defined in the current crate. You cannot implement third-party traits on third-party types.

When calling methods defined in a trait, be sure to make the trait accessible.

```rust
let mut f = std::fs::File::open("foo.txt").ok().expect("Couldn’t open foo.txt");
let buf = b"whatever"; //  buf: &[u8; 8]
let result = f.write(buf);
# result.unwrap();
```

Here is the error:

```
error: type `std::fs::File` does not implement any method in scope named `write`
let result = f.write(buf);
               ^~~~~~~~~~
```

We need to use this Write trait first:

```rust
use std::io::Write;

let mut f = std::fs::File::open("foo.txt").expect("Couldn’t open foo.txt");
let buf = b"whatever";
let result = f.write(buf);
# result.unwrap(); // ignore the error
```

This compiles without errors.


## default method of trait


```rust
trait Foo {
    fn is_valid(&self) -> bool;

    fn is_invalid(&self) -> bool { !self.is_valid() }
}
```

`is_invalid` is the default method, implementors of `Foo` are not required to implement it, if they choose to implement it, it will override its default behavior.

## trait inheritance

```rust
trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foobar(&self);
}
```

Thus implementors of `FooBar` must also implement `Foo`:

```rust
struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}
```

## derive attribute

**Rust** provides an attribute `derived` to automatically implement some traits, so as to avoid repetitive and tedious implementation of them. The traits that can be used by `derived` include: `Clone`, `Copy`, `Debug`, ` Default`, `Eq`, `Hash`, `Ord`, `PartialEq`, `PartialOrd`

```rust
#[derive(Debug)]
struct Foo;

fn main() {
    println!("{:?}", Foo);
}
```

## impl Trait
Starting from version 1.26, Rust provides `impl Trait`, which is equivalent to Scala's `Existential Type'.

In the following syntax, `fn foo()` will return a trait that implements `Trait`.

```rust
//before
fn foo() -> Box<Trait> {
    // ...
}

//after
fn foo() -> impl Trait {
    // ...
}
```

Compared with the way of writing before version 1.25, the new way of writing will be more conducive to development and execution efficiency in many occasions.

#### Common use cases for impl Trait

```rust
trait Trait {
    fn method(&self);
}

impl Trait for i32 {
    // implementation goes here
}

impl Trait for f32 {
    // implementation goes here
}
```

Using Box means that even if the returned content is fixed, dynamic memory allocation will be used. Use `impl Trait` to avoid using Box.

```rust
//before
fn foo() -> Box<Trait> {
    Box::new(5) as Box<Trait>
}

//after
fn foo() -> impl Trait {
    5
}
```

#### Other benefit use cases

Closure:
```rust
// before
fn foo() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// after
fn foo() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

Pass parameters:
```rust
// before
fn foo<T: Trait>(x: T) {

// after
fn foo(x: impl Trait) {
```
