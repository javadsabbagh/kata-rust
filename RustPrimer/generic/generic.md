# Generics


In our programming, we usually have the need to write a function with the same function for multiple types of data, such as the addition of two numbers. We hope that this function supports both i8, i16, i32....float64, etc., and even For custom types, in programming languages that do not support generics, we usually have to write a function for each type, and usually the function names must be different, for example:

```rust
fn add_i8(a:i8, b:i8) -> i8 {
	a + b
}
fn add_i16(a:i16, b:i16) -> i16 {
	a + b
}
fn add_f64(a:f64, b:f64) -> f64 {
	a + b
}

// Various other add functions
//...

fn main() {
	println!("add i8: {}", add_i8(2i8, 3i8));
	println!("add i16: {}", add_i16(20i16, 30i16));
	println!("add f64: {}", add_f64(1.23, 1.23));
}
```

If there are many places that need to support multiple types, then the amount of code will be very large, and the code will be very bloated, and programming will really become a hard work of moving bricks, which is boring and tedious: D.
People who have learned C++ may easily understand generics, but this tutorial is aimed at Rust beginners, so we will not compare C++ generics, polymorphism and Rust, so as not to increase the complexity of learning and unnecessary troubles. This makes it easier for Rust beginners to understand and accept Rust generics.


## concept

Generic programming is a style or paradigm of programming languages. Allows programmers to use some later-specified types when writing code in a strongly typed programming language, and specify these types as parameters at instantiate (in Rust, sometimes types can also be deduced by the compiler) . Various programming languages and their compilers and operating environments have different support for generics. Ada, Delphi, Eiffel, Java, C#, F#, Swift, and Visual Basic .NET call it generics; ML, Scala and Haskell call it parametric polymorphism (parametric polymorphism); C++ and D call it template. The widely influential 1994 edition of Design Patterns called this a parameterized type.

> Tips:
>The above concepts are taken from ["Wikipedia - Generics"](https://zh.wikipedia.org/wiki/%E6%B3%9B%E5%9E%8B)

When programming, we often use polymorphism. In layman's terms, polymorphism is like the gun barrel of a tank. It can launch ordinary ammunition, guided shells (missiles), depleted uranium armor-piercing shells, and even sub-munitions. Everyone doesn't want to be on the tank for every shell. Installing a dedicated barrel separately, even if the manufacturer is willing, the gunner is not willing, it is exhausting. So in programming development, we also need such a "universal barrel", this "universal barrel" is polymorphism.

What you need to know is that generics are a kind of polymorphism.

The main purpose of generics is to provide programmers with the convenience of programming, reduce the bloated code, and at the same time greatly enrich the expressive ability of the language itself, and provide programmers with a suitable barrel. Think about how exciting it is for one function to replace dozens or even hundreds of functions.
Generics can be understood as collection types with certain functional commonality, such as i8, i16, u8, f32, etc. can all support add, and even two struct Point types can be added to form a new Point.

Let us first take a look at the common generic Option<T> in the standard library, its prototype definition:

```rust
enum Option<T> {
	Some(T),
	None,
}
```

T is a generic parameter, where T can be replaced with any letter you like from A-Z. But customarily, we use T to represent Type and E to represent Error. T will be instantiated when it is actually used:

```rust
let a = Some(100.111f32);
```

The compiler will automatically deduce that a is of type Option<f32>, that is to say, T in Option is of type f32 here.

Of course, you can also explicitly declare the type of a, but it must be guaranteed to be the same as the type of the rvalue, otherwise the compiler will report a "mismatched types" type mismatch error.

```rust
let a:Option<f32> = Some(100.111); // Compile automatically deduces 100.111 in the rvalue as f32 type.
let b:Option<f32> = Some(100.111f32);
let c:Option<f64> = Some(100.111);
let d:Option<f64> = Some(100.111f64);
```


### Generic functions
So far, we have learned the definition and simple use of generics.
Now let's rewrite the add operation with a function:

```rust
use std::ops::Add;

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
	a + b
}

fn main() {
	println!("{}", add(100i32, 1i32));
	println!("{}", add(100.11f32, 100.22f32));
}
```

> **Output:**
>101
>200.33

```add<T: Add<T, Output=T>>(a:T, b:T) -> T``` is our generic function, and the return value is also generic T, the meaning in Add<> It can be ignored temporarily. The general meaning is that as long as the parameter type implements the Add trait, it can be passed to our add function. Because our add function has an addition + operation, the parameter type must be additive. Yes, that is, the Add trait must be implemented (refer to std::ops::Add for details).

### Custom Type
In the above example, add is the basic data type built into the language. Of course, we can also implement the add operation for our own custom data structure type.

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// implement Add trait for Point
impl Add for Point {
     type Output = Point; //Execution return value type is Point
     fn add(self, p: Point) -> Point {
         Point {
             x: self.x + p.x,
             y: self.y + p.y,
         }
     }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
a + b
}

fn main() {
println!("{}", add(100i32, 1i32));
println!("{}", add(100.11f32, 100.22f32));

let p1 = Point{x: 1, y: 1};
let p2 = Point{x: 2, y: 2};
println!("{:?}", add(p1, p2));
}
```

> **Output:**
>101
200.33
Point { x: 3, y: 3 }

The above example is a little more complicated, but we added a custom type, and then let the add function still work on it. If you are not familiar with traits, please refer to the relevant chapters of traits.

You may wonder, can we make Point generic, so that the x and y of Point can also support float type or other types, the answer is of course yes.

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { //The restriction type T must implement the Add trait, otherwise the + operation cannot be performed.
     x: T,
     y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
     type Output = Point<T>;

     fn add(self, p: Point<T>) -> Point<T> {
         Point {
             x: self.x + p.x,
             y: self.y + p.y,
         }
     }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
a + b
}

fn main() {
let p1 = Point{x: 1.1f32, y: 1.1f32};
let p2 = Point{x: 2.1f32, y: 2.1f32};
println!("{:?}", add(p1, p2));

let p3 = Point{x: 1i32, y: 1i32};
let p4 = Point{x: 2i32, y: 2i32};
println!("{:?}", add(p3, p4));
}
```

> **Output:**
>Point { x: 3.2, y: 3.2 }
Point { x: 3, y: 3 }

The above example is more complicated. We not only allow the custom Point type to support the add operation, but also make it generic for Point.

When ```let p1 = Point{x: 1.1f32, y: 1.1f32};```, the T of Point is deduced as f32 type, so the x and y properties of Point become f32 type. Because of p1.x+p2.x, the T type must support Add trait.

### Summarize
The dozens of lines of code above achieve the effect that can only be achieved by a non-generic language with hundreds or even thousands of lines of code, which shows the power of generics.

### Exercises

#### 1. Generic lines iterator

##### Problem Description
Sometimes we may do some text analysis work, and the data may come from external or built-in text in the program.

Please implement a `parse` function that only accepts a lines iterator as an argument and outputs each line.

It is required to output both built-in text and file content.

##### Call method and output reference

```
let lines = "some\nlong\ntext".lines()
parse(do_something_or_nothing(lines))
```

```
some
long
text
```

```
use std::fs:File;
use std::io::prelude::*;
use std::io::BufReader;
let lines = BufReader::new(File::open("/etc/hosts").unwrap()).lines()
parse(do_some_other_thing_or_nothing(lines))
```

```
127.0.0.1 localhost.localdomain localhost
::1 localhost.localdomain localhost
...
```

##### Hint
The traits such as AsRef, Borrow, etc. introduced in the chapter `Several common traits in the type system` of this book should come in handy.
