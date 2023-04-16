# Composite type

## Tuple

In other languages, you may have heard the word tuple, which means an ordered group of data of a fixed size and type. In Rust, the situation is not fundamentally different. However, Rust provides us with a series of simple and convenient syntax so that we can use it better.

```rust
let y = (2, "hello world");
let x: (i32, &str) = (3, "world hello");

// Then, you can access them in a very simple way:

// use let expression
let (w, z) = y; // w=2, z="hello world"

// use subscript

let f = x.0; // f = 3
let e = x.1; // e = "world hello"
```

## structure (struct)

In Rust, a struct is a concept similar to a `tuple`. We can also aggregate some common data and attributes together to form a structure.

The difference is that Rust's structure has three basic forms.

### Named structure

As for this structure, it can be roughly regarded as such a declaration form:

```rust
struct A {
    attr1: i32,
    atrr2: String,
}
```

Inside each member has its own name and type.

### Tuple type structure

Tuple-type structures use parentheses, similar to `tuple`.

```rust
struct B(i32, u16, bool);
```

It can be regarded as a tuple with a name, and its specific usage is basically similar to that of a general tuple.

### empty structure

A structure can also have no members inside.

```rust
struct D;
```

The memory footprint of an empty structure is 0. But we can still implement its "member functions" for such types.

But so far, in versions prior to 1.9, you can't put parentheses after an empty struct.
If written like this, it will cause this part of the old compiler to compile errors:

```rust
struct C {

}
```

### Implement the structure (impl)

Rust has no inheritance. It and Golang have chosen trait (Golang is called Interface) as the basis for polymorphism. However, if we want to write some specialized member functions for a structure, how should we write them?

Answer: impl

talk is cheap , give a chestnut:

```rust
struct Person {
    name: String,
}

impl Person {
    fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
        }
    }

    fn greeting(&self) {
        println!("{} say hello .", self.name);
    }
}

fn main() {
    let peter = Person::new("Peter");
    peter.greeting();
}
```

Seeing `self`, the Python programmer smiled unkindly.

Let's analyze it. In the `impl` above, new is called by the Person structure itself, and its characteristic is the call of `::`. Java programmers stand up: class functions! And `greeting` with `self`, is more like a member function.

Well, the answer is correct, but no extra points.

### Discussion about various refs

Rust has strict security controls on the code, so there is a concept of ownership and borrowing for a variable. Ownership can only be held by one person at a time, mutable references can only be held by one instance at the same time, and immutable references can be held by multiple instances. At the same time all properties are moved, called `move` in Rust.

The above is the basic concept of ownership. In fact, during the entire software operation cycle, the conversion of ownership is an extremely annoying and cumbersome thing, especially for those students who are new to Rust. Similarly, Rust's structure, which is the cornerstone of its type system, also has relatively strict ownership control restrictions. Specifically, there are two situations you need to consider with respect to ownership of structs.

#### ref and owner of fields

In the above structure, we have defined many structures, but as you can see, each field of the structure is completely its own. That is to say, the owner of each field is this structure. The lifetime of each field will eventually not exceed this structure.

But sometimes, what if I just want to hold the value of a (mutable) reference?
The following code:

```rust
struct RefBoy {
    loc: &i32,
}
```

At this point you will get a compile error:

```
<anon>:6:14: 6:19 error: missing lifetime specifier [E0106]
<anon>:6         loc: & i32,
```

At this time, you will hold a reference to a value, because its own life cycle is outside this structure, so for this structure, it cannot accurately determine the life cycle of this reference, which is compiled in Rust device is not acceptable.
Therefore, at this time, we need to artificially write a life cycle for this structure, and explicitly indicate the life cycle of this reference. It is written as follows:

```rust
struct RefBoy<'a> {
    loc: &'a i32,
}
```

Here is an explanation of this symbol `<>`, which represents a `belongs to` relationship, no matter it describes *life cycle* or *generic*. That is: `RefBoy in 'a`. Finally, we can come to a conclusion that the life cycle of `RefBoy` structure must not be longer than `'a`.

After writing this, some people may still be confused about the life cycle and do not understand the reason. In fact, you only need to know two things:

1. The reference fields in the structure must have an explicit life cycle
2. A structure with an explicitly written life cycle must have its own life cycle less than or equal to any of its explicitly written life cycles

Regarding the second point, in fact, multiple life cycles can be written, separated by `,`.

Note: Life cycle and generics are written in `<>`, first life cycle and then generics, separated by `,`.

#### Three kinds of self in impl

We know earlier that in Rust, member methods can be added to a structure through impl. At the same time, we have also seen keywords such as `self`. At the same time, this self also has several situations that require you to remember carefully.

There are three common forms of self in impl: `self`, `&self`, `&mut self`, let’s talk about them separately.

##### moved self

Just like the impl in the above example, we implement a function that takes `self` as the first parameter, but such a function is actually problematic.
The problem is with Rust's ownership transfer mechanism.

I once saw a joke about Rust: "you call someone else, and then you're not yours".

For example, the following code will report an error:

```rust
struct A {
    a: i32,
}
impl A {
    pub fn show(self) {
        println!("{}", self.a);
    }
}

fn main() {
    let ast = A{a: 12i32};
    ast.show();
    println!("{}", ast.a);
}
```

mistake:

```
13:25 error: use of moved value: `ast.a` [E0382]
<anon>:13     println!("{}", ast.a);
```

why? Because of Rust itself, when you call a function, if it is not a reference, then undoubtedly, this parameter will be eaten by this function, that is, its owner will be moved to the parameter of this function. In the same way, `self` in `impl`, if what you write is not a reference, will also be moved by default!

So how to avoid this situation? The answer is `Copy` and `Clone`:

```rust
#[derive(Copy, Clone)]
struct A {
    a: i32,
}
```

If written in this way, the compilation will pass. But writing this way actually has its flaws. The disadvantage is: `Copy` or `Clone` will bring a certain amount of runtime overhead! In fact, `self` being moved is actually a relatively seldom-used situation. More often, what we need is `ref` and `ref mut`.

###### ref and ref mut

The writing method of `ref` and `mut ref` is similar to the writing method of the moved `self`, except that there is an additional reference modifier symbol. There are examples above, so I won’t say much.

One thing to note is that you cannot call a `&mut ref` inside a `&self` method, not under any circumstances!

However, the reverse is possible. code show as below:

```rust
#[derive(Copy, Clone)]
struct A {
    a: i32,
}
impl A {
    pub fn show(&self) {
        println!("{}", self.a);
        // compile error: cannot borrow immutable borrowed content `*self` as mutable
        // self.add_one();
    }
    pub fn add_two(&mut self) {
        self.add_one();
        self.add_one();
        self.show();
    }
    pub fn add_one(&mut self) {
        self.a += 1;
    }
}

fn main() {
    let mut ast = A{a: 12i32};
    ast.show();
    ast.add_two();
}
```

It should be noted that once your struct holds a mutable reference, you can only change it in the implementation of `&mut self`!

Rust allows us to flexibly implement what you want on a struct, which has undoubtedly greatly improved the degree of freedom in programming.

As for the more advanced usage of traits and generics, we will introduce them in detail in future chapters.

## enumeration type enum

Rust's enumeration (`enum`) type is a bit close to the C language enumeration, but it is more powerful. In fact, it is an algebraic data type (Algebraic Data Type).

For example, here is an enumeration representing the four directions:

```rust
enum Direction {
    West,
    North,
    South,
    East,
}
```

However, rust's enums can do more than C's.
For example, the enumeration can actually contain some specific data information you need!
This is something that regular enumerations can't do, it's more like an enumeration class, isn't it?

```rust
enum SpecialPoint {
    Point(i32, i32),
    Special(String),
}
```

You can also name the fields inside, such as

```rust
enum SpecialPoint {
    Point {
        x: i32,
        y: i32,
    },
    Special(String),
}
```

### Using enums

Different from the member access symbol `.` of struct, if an enumeration type wants to access its members, pattern matching must be used almost without exception. Also, you can write a `Direction::West`, but you can't yet write `Direction.West`, unless you explicitly `use` it. Although the compiler is smart enough to spot your carelessness.


About pattern matching, I won't say too much, let's give a chestnut

```rust
enum SpecialPoint {
    Point(i32, i32),
    Special(String),
}

fn main() {
    let sp = SpecialPoint::Point(0, 0);
    match sp {
        SpecialPoint::Point(x, y) => {
            println!("I'am SpecialPoint(x={}, y={})", x, y);
        }
        SpecialPoint::Special(why) => {
            println!("I'am Special because I am {}", why);
        }
    }
}
```

Na na na, this is the pattern matching value.
Of course, `enum` can actually `impl`, and I don’t tell ordinary people!

For enums with named fields, field names can be specified when pattern matching

```rust
match sp {
    SpecialPoint::Point { x: x, y: y } => {
        // ...
    },
    SpecialPoint::Special(why) => {}
}
```

For enumeration types with field names, the pattern matching syntax is the same as when matching `struct`. like

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 1, y: 2 };

let Point { x: x, y: y } = point;
// or
let Point { x, y } = point;
// or
let Point { x: x, .. } = point;
```

The syntax of pattern matching is consistent with `if let` and `let`, so the same syntax is supported as seen in the following content.
