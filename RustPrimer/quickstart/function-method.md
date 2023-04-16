# Functions and methods

## function

To declare a function, you need to use the keyword `fn` followed by the function name, such as

```rust
fn add_one(x: i32) -> i32 {
     x + 1
}
```

The type of the function parameter cannot be omitted, there can be multiple parameters, but at most one value can be returned,
To return early use the `return` keyword. The Rust compiler will warn about unused functions,
Dead code checking can be disabled using the attribute `#[allow(dead_code)]`.

Rust has a special feature for diverging functions that don't return:

```rust
fn diverges() -> ! {
     panic!("This function never returns!");
}
```

where `panic!` is a macro that crashes the current thread of execution and prints the given message. The return type `!` can be used as any type:

```rust
let x: i32 = diverges();
let y: String = diverges();
```

## anonymous function

Rust uses closures to create anonymous functions:

```rust
let num = 5;
let plus_num = |x: i32| x + num;
```

The closure `plus_num` borrows the `let` binding `num` in its scope. If you want the closure to take ownership,
You can use the `move` keyword:

```rust
let mut num = 5;

{
     let mut add_num = move |x: i32| num += x; // The closure acquires the ownership of num through move

     add_num(5);
}

// The following num can continue to be used after being moved because it implements the Copy feature
// For details, see the Ownership (Owership) chapter
assert_eq!(5, num);
```

## Higher order functions

Rust also supports high order functions, which allow closures to be passed as arguments to generate new functions:

```rust
fn add_one(x: i32) -> i32 { x + 1 }

fn apply<F>(f: F, y: i32) -> i32
     where F: Fn(i32) -> i32
{
     f(y) * y
}

fn factory(x: i32) -> Box<Fn(i32) -> i32> {
     Box::new(move |y| x + y)
}

fn main() {
     let transform: fn(i32) -> i32 = add_one;
     let f0 = add_one(2i32) * 2;
     let f1 = apply(add_one, 2);
     let f2 = apply(transform, 2);
     println!("{}, {}, {}", f0, f1, f2);

     let closure = |x: i32| x + 1;
     let c0 = closure(2i32) * 2;
     let c1 = apply(closure, 2);
     let c2 = apply(|x| x + 1, 2);
     println!("{}, {}, {}", c0, c1, c2);

     let box_fn = factory(1i32);
     let b0 = box_fn(2i32) * 2;
     let b1 = (*box_fn)(2i32) * 2;
     let b2 = (&box_fn)(2i32) * 2;
     println!("{}, {}, {}", b0, b1, b2);

     let add_num = &(*box_fn);
     let translate: &Fn(i32) -> i32 = add_num;
     let z0 = add_num(2i32) * 2;
     let z1 = apply(add_num, 2);
     let z2 = apply(translate, 2);
     println!("{}, {}, {}", z0, z1, z2);
}
```

## method

Rust uses the `impl` keyword to implement method call syntax (method call syntax) on `struct`, `enum` or `trait` objects.
The first parameter of an associated function is usually the `self` parameter, and there are 3 variants:
* `self`, allowing the implementor to move and modify the object, the corresponding closure feature is `FnOnce`.
* `&self`, the implementer is neither allowed to move the object nor modify it, and the corresponding closure feature is `Fn`.
* `&mut self`, allows the implementer to modify the object but does not allow movement, and the corresponding closure feature is `FnMut`.

An associated function without a `self` parameter is called a static method.

```rust
struct Circle {
     x: f64,
     y: f64,
     radius: f64,
}

impl Circle {
     fn new(x: f64, y: f64, radius: f64) -> Circle {
         Circle {
             x: x,
             y: y,
             radius: radius,
         }
     }

     fn area(&self) -> f64 {
         std::f64::consts::PI * (self. radius * self. radius)
     }
}

fn main() {
     let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
     println!("{}", c.area());

     // use associated function and method chaining
     println!("{}", Circle::new(0.0, 0.0, 2.0).area());
}
```
