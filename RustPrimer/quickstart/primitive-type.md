# Variable binding and primitive types

## Variable Binding
Rust uses the let keyword for variable binding.

```rust
fn main() {
     let a1 = 5;
     let a2:i32 = 5;
     assert_eq!(a1, a2);
     //let binding integer variable default type inference is i32

     let b1:u32 = 5;
     //assert_eq!(a1, b1);
     //Removing the above comment will report an error because the type does not match
     //errer: mismatched types
}
```

The function of the assert_eq! macro here is to judge whether the two parameters are equal, but if the two parameters do not match, an error will be reported even if the literal values are equal.

## Mutable Binding
When declaring a variable in rust, add the mut keyword in front of the variable, and the variable will become a variable binding variable.

```rust
fn main() {
     let mut a: f64 = 1.0;
     let b = 2.0f32;

     //Change the binding of a
     a = 2.0;
     println!("{:?}", a);

     // Rebind as immutable
     let a = a;

     // cannot be assigned
     //a = 3.0;

     //type mismatch
     //assert_eq!(a, b);
}
```

The b variable here is bound to 2.0f32. This is the syntax for explicit marking of value types in Rust, specified in the form of `value`+`type`.

**For example:**
Fixed size type:
> 1u8 1i8
> 1u16 1i16
> 1u32 1i32
> 1u64 1i64

variable size type:
> 1usize 1isize

Float type:
> 1f32 1f64

## let deconstruction
Why use a let binding expression when declaring a variable in Rust?
That's because let binding expressions are more expressive, and let expressions are actually a type of pattern matching.

**For example:**

```rust
fn main() {
     let (a, mut b): (bool, bool) = (true, false);
     println!("a = {:?}, b = {:?}", a, b);
     //a immutable binding
     //a = false;

     //b mutable binding
     b = true;
     assert_eq!(a, b);
}
```

The bool is used here, which has only two values of true and false, which are usually used for logical judgment.

## Primitive types

Rust's built-in primitive types (primitive types) are as follows:

* Boolean type: There are two values `true` and `false`.
* Character type: represents a single Unicode character, stored as 4 bytes.
* Numerical types: divided into signed integers (`i8`, `i16`, `i32`, `i64`, `isize`),
Unsigned integers (`u8`, `u16`, `u32`, `u64`, `usize`) and floating point numbers (`f32`, `f64`).
* String type: The lowest level is the variable length type `str`, and the more commonly used string slice `&str` and heap allocated string `String`,
Among them, the string slice is statically allocated, has a fixed size, and is immutable, while the heap allocated string is variable.
* Array: has a fixed size, and the elements are all of the same type, which can be expressed as `[T; N]`.
* Slicing: refers to part of the data of an array and does not need to be copied, which can be expressed as `&[T]`.
* Tuple: An ordered list with a fixed size, each element has its own type, and the value of each element is obtained by destructuring or indexing.
* Pointer: The bottom layer is the raw pointer `*const T` and `*mut T`, but it is unsafe to dereference them and must be placed in the `unsafe` block.
* Function: A variable with a function type is essentially a function pointer.
* Metatype: namely `()`, its only value is also `()`.

```rust
// boolean type
let t = true;
let f: bool = false;

// char type
let c = 'c';

// numeric types
let x = 42;
let y: u32 = 123_456;
let z: f64 = 1.23e+2;
let zero = z.abs_sub(123.4);
let bin = 0b1111_0000;
let oct = 0o7320_1546;
let hex = 0xf23a_b049;

// string types
let str = "Hello, world!";
let mut string = str.to_string();

// arrays and slices
let a = [0, 1, 2, 3, 4];
let middle = &a[1..4];
let mut ten_zeros: [i64; 10] = [0; 10];

// tuples
let tuple: (i32, &str) = (50, "hello");
let (fifty, _) = tuple;
let hello = tuple.1;

// raw pointers
let x = 5;
let raw = &x as *const i32;
let points_at = unsafe { *raw };

// functions
fn foo(x: i32) -> i32 { x }
let bar: fn(i32) -> i32 = foo;
```

There are a few points that need special attention:

* Numeric types can use the `_` delimiter to increase readability.
* Rust also supports the single-byte character `b'H'` and the single-byte string `b"Hello"`, limited to ASCII characters only.
In addition, you can also use the `r#"..."#` tag to represent a raw string without escaping special characters.
* Using the `&` symbol to convert `String` type to `&str` type is cheap,
But using the `to_string()` method to convert `&str` to `String` type involves allocating memory,
Don't do this unless absolutely necessary.
* The length of the array is immutable, and the dynamic array is called Vec (vector), which can be created using the macro `vec!`.
* Tuples can use `==` and `!=` operators to determine whether they are the same.
* Arrays with no more than 32 elements and tuples with no more than 12 elements are automatically copied when the value is passed.
* Rust does not provide implicit conversion between native types, only explicit conversion can be done using the `as` keyword.
* You can use the `type` keyword to define an alias of a type, and should use camelCase.

```rust
// explicit conversion
let decimal = 65.4321_f32;
let integer = decimal as u8;
let character = integer as char;

// type aliases
type NanoSecond = u64;
type Point = (u8, u8);
```
