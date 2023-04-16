# Structs and enumerations

## structure

A struct is a record type that contains a name for each field.
Each struct also has a name, usually beginning with a capital letter and using camelCase.
The tuple structure (tuple struct) is composed of a mixture of tuple and structure. The tuple structure has a name.
But its domain doesn't. When a tuple structure has only one field, it is called a newtype.
A structure without any fields is called a unit-like struct.
The value in the structure is immutable by default, you need to add `mut` to the structure to make it variable.

```rust
// structs
struct Point {
  x: i32,
  y: i32,
}
let point = Point { x: 0, y: 0 };

// tuple structs
struct Color(u8, u8, u8);
let android_green = Color(0xa4, 0xc6, 0x39);
let Color(red, green, blue) = android_green;

// A tuple struct's constructors can be used as functions.
struct Digit(i32);
let v = vec![0, 1, 2];
let d: Vec<Digit> = v.into_iter().map(Digit).collect();

// newtype: a tuple struct with only one element
struct Inches(i32);
let length = Inches(10);
let Inches(integer_length) = length;

// unit-like structs
struct EmptyStruct;
let empty = EmptyStruct;
```

A `struct` containing `..` can be used to copy some values from other structs or ignore some fields during destructuring:

```rust
#[derive(Default)]
struct Point3d {
     x: i32,
     y: i32,
     z: i32,
}

let origin = Point3d::default();
let point = Point3d { y: 1, .. origin };
let Point3d { x: x0, y: y0, .. } = point;
```

Note that Rust does not support field mutability at the language level, so you cannot write:

```rust
struct Point {
     mut x: i32,
     y: i32,
}
```

This is because mutability is a property of the binding, not of the struct itself. This can be simulated using `Cell<T>`:

```rust
use std::cell::Cell;

struct Point {
     x: i32,
     y: Cell<i32>,
}

let point = Point { x: 5, y: Cell::new(6) };

point.y.set(7);
```

In addition, fields of structs are private by default outside the module (mod) in which they reside, and can be made public using the `pub` keyword.

```rust
mod graph {
    #[derive(Default)]
    pub struct Point {
        pub x: i32,
        y: i32,
    }

    pub fn inside_fn() {
        let p = Point {x:1, y:2};
        println!("{}, {}", p.x, p.y);
    }
}

fn outside_fn() {
    let p = graph::Point::default();
    println!("{}", p.x);
    // println!("{}", p.y);
    // field `y` of struct `graph::Point` is private
}
```

## enumeration
Rust has a collection type, called an enumeration (enum), which represents a collection of sub-datatypes.
The sub-data structure can be empty - if all sub-data structures are empty, it is equivalent to enum in C language.
We need to use `::` to get the name of each element.

```rust
// enums
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

let x: Message = Message::Move { x: 3, y: 4 };
```

Like structures, elements in enumerations cannot be compared using relational operators by default (such as `==`, `!=`, `>=`),
Binary operators like `+` and `*` are also not supported, you need to implement it yourself, or use `match` for matching.

An enumeration is also private by default, and if it is made public using `pub`, its elements are also public by default.
This is different from structs: even though a struct is public, its fields are still private by default. Public/private here is still
is outside the module in which it is defined. Additionally, enums and structs can also be recursive.
