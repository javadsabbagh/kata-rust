# operator overloading

Rust allows us to overload certain operators, most of which are implemented by overloading traits under `std::ops`.

## Overloaded addition

Let's now implement a castration version [plural] that only supports addition AD%A6%29):

```rust
use std::ops::Add;

#[derive(Debug)]
struct Complex {
     a: f64,
     b: f64,
}

impl Add for Complex {
     type Output = Complex;
     fn add(self, other: Complex) -> Complex {
         Complex {a: self.a+other.a, b: self.b+other.b}
     }
}

fn main() {
     let cp1 = Complex{a: 1f64, b: 2.0};
     let cp2 = Complex{a: 5.0, b: 8.1};
     let cp3 = cp1 + cp2;
     print!("{:?}", cp3);
}
```

output:

```
Complex { a: 6, b: 10.1}
```

Here we implement the `std::ops::Add` trait. At this time, some students slapped their heads, so that’s the case, that’s right... In fact, most of the operators in Rust are syntactic sugar of traits under `std::ops`!

Let's take a look at the specific structure of `std::ops::Add`

```rust
impl Add<i32> for Point {
     type Output = f64;

     fn add(self, rhs: i32) -> f64 {
         // add an i32 to a Point and get an f64
     }
}
```

## Magical Output and dynamic distribution
Some students will ask, why is this `Output` swollen? Answer, type conversion yo dear!
To give an inappropriate chestnut, we will have a formula like `0.5+0.5=1` in reality, which can be described in Rust language as follows: two `f32` are added to get an `i8`. Obviously, Output is designed for this situation.

Still look at the code:

```rust
use std::ops::Add;

#[derive(Debug)]
struct Complex {
     a: f64,
     b: f64,
}

impl Add for Complex {
     type Output = Complex;
     fn add(self, other: Complex) -> Complex {
         Complex {a: self.a+other.a, b: self.b+other.b}
     }
}

impl Add<i32> for Complex {
     type Output = f64;
     fn add(self, other: i32) -> f64 {
         self.a + self.b + (other as f64)
     }
}

fn main() {
     let cp1 = Complex{a: 1f64, b: 2.0};
     let cp2 = Complex{a: 5.0, b: 8.1};
     let cp3 = Complex{a: 9.0, b: 20.0};
     let complex_add_result = cp1 + cp2;
     print!("{:?}\n", complex_add_result);
     print!("{:?}", cp3 + 10i32);
}
```

Output result:

```
Complex { a: 6, b: 10.1 }
39
```

## Restrictions on generic types

Rust's operators are based on the trait system. Similarly, operators can be regarded as a restriction on generic types. We can require that `generic type T must implement trait Mul<Output=T>`.
So, we got the following code:

```rust
use std::ops::Mul;

trait HasArea<T> {
     fn area(&self) -> T;
}

struct Square<T> {
     x: T,
     y: T,
     side: T,
}

impl<T> HasArea<T> for Square<T>
         where T: Mul<Output=T> + Copy {
     fn area(&self) -> T {
         self.side *self.side
     }
}

fn main() {
     let s = Square {
         x: 0.0f64,
         y: 0.0f64,
         side: 12.0f64,
     };

     println!("Area of s: {}", s.area());
}
```

For trait `HasArea<T>` and struct `Square<T>`, we restrict `T` to implement multiplication through `where T: Mul<Output=T> + Copy`. At the same time, Copy restricts Rust from moving self.side into the return value.

The writing method is simple, relaxed and pleasant.
