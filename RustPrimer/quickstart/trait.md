# Features

## Features and interfaces
To describe an abstract interface that a type can implement,
Rust introduces traits to define function type signatures:

```rust
trait HasArea {
    fn area(&self) -> f64;
}

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

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}
```

Among them, the generic parameter `T` in the function `print_area()` is added a trait constraint named `HasArea`,
Used to ensure that any type that implements `HasArea` will have a `.area()` method.
If you need multiple trait bounds, you can use `+`:

```rust
use std::fmt::Debug;

fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn bar<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}
```

The second example uses a more flexible `where` clause, which also allows the left side of the qualification to be of any type,
rather than just type parameters.

Methods defined in an attribute are called default methods and can be overridden by the implementation of that attribute.
In addition, inheritance can also exist between attributes:

```rust
trait Foo {
    fn foo(&self);

    // default method
    fn bar(&self) { println!("We called bar."); }
}

// inheritance
trait FooBar : Foo {
    fn foobar(&self);
}

struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}
```

If two methods with different characteristics have the same name, you can use the universal function call syntax:

```rust
// short-hand form
Trait::method(args);

// expanded form
<Type as Trait>::method(args);
```

A few restrictions on implementing features:

* If a feature is not in the current scope, it cannot be implemented.
* Both traits and `impl` only work within the current crate.
* Generic functions with attribute constraints are implemented using monomorphization (monomorphization),
So it is statically dispatched.

Here are a few very useful standard library features:

* `Drop` provides the function of executing code when a value exits the scope, it only has a `drop(&mut self)` method.
* `Borrow` is used to create a data structure that treats owned and borrowed values as equivalent.
* `AsRef` is used to convert a value to a reference in generics.
* `Deref<Target=T>` is used to automatically convert the value of `&U` type to `&T` type.
* `Iterator` is used to implement iterators on collections and lazy value generators.
* `Sized` is used to mark a type with a fixed length at runtime, while slices and properties with an indefinite length must be placed behind the pointer to make the length known at runtime,
For example `&[T]` and `Box<Trait>`.

## Generics and polymorphism

Generics are called parametric polymorphism in type theory,
Means a function or type that can have multiple forms for a given argument. Let's look at an example of generics in Rust:

The definition of Option in the rust standard library:

```rust
enum Option<T> {
    Some(T),
    None,
}
```
Typical usage of Option:
```rust
let x: Option<i32> = Some(5);
let y: Option<f64> = Some(5.0f64);
```

The `<T>` part indicates that it is a generic data type. Of course, generic parameters can also be used for function parameters and structure fields:

```rust
// generic functions
fn make_pair<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}
let couple = make_pair("man", "female");

// generic structs
struct Point<T> {
    x: T,
    y: T,
}
let int_origin = Point { x: 0, y: 0 };
let float_origin = Point { x: 0.0, y: 0.0 };
```

For polymorphic functions, there are two dispatch mechanisms: static dispatch and dynamic dispatch.
The former is similar to the template of C++, Rust will generate a special function suitable for the specified type, and then replace it at the called position,
The advantage is that the function is allowed to be called inline, which runs faster, but it will cause code bloat;
The latter is similar to `interface` in Java or Go, and Rust implements it by introducing trait objects,
Look up the virtual table (vtable) at runtime to select the method to execute. The property object `&Foo` has the same name as the property `Foo`,
Created by casting or coercing a pointer to a concrete type.

Of course, traits can also accept generic parameters. However, it is often better to use an associated type:

```rust
// use generic parameters
trait Graph<N, E> {
    fn has_edge(&self, &N, &N) -> bool;
    fn edges(&self, &N) -> Vec<E>;
}

fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {

}

// use associated types
trait Graph {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> uint {

}

struct Node;

struct Edge;

struct SimpleGraph;

impl Graph for SimpleGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {

    }

    fn edges(&self, n: &Node) -> Vec<Edge> {

    }
}

let graph = SimpleGraph;
let object = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;

```

