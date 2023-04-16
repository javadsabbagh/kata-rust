# native type

Like other modern programming languages, Rust provides a basic set of types, which we generally call *primitive types*. Its powerful type system is based on these native types. Therefore, before writing Rust code, you must have a certain understanding of Rust's native types.

## bool

Rust comes with a `bool` type whose possible values are `true` or `false`.
We can declare it this way:

```rust
let is_she_love_me = false;
let mut is_he_love_me: bool = true;
```

Of course, the place where the bool type is used the most is in the `if expression`.

## char

In Rust, a `char` type represents a *Unicode* character, which means that a char that represents a character (8bit) in some languages is actually four bytes (32bit) in Rust.
At the same time, we can freely assign all kinds of strange non-Chinese characters to a char type. It should be noted that in Rust, we need to use `'` to represent a char. If you use `"`, what you get is actually a `&'static str`.

```rust
let c = 'x';
let cc = 'King';
```

## Number type

Unlike other C-like languages, Rust uses a *symbol+digits* way to represent its basic numeric types. Maybe you are used to representations such as `int`, `double`, `float`, Rust’s representation requires you to get used to it a little bit.

The symbols you can use are `i`, `f`, `u`

The number of bits you can use, of course, is 2 to the power of n, namely `8`, `16`, `32`, `64` and `size`.

You can combine them to form types such as `i32`, `u16`, etc.

Of course, such a combination is not free, because the floating-point type can only be represented by at least 32 bits, so it can only be represented by `f32` and `f64`.

### Adaptive type

After reading the above, you must be curious about `isize` and `usize`. What are these two here for? Well, these two actually depend on the number of bits in your operating system. Simple and rude, for example, 64-bit computers are 64-bit, 32-bit computers are 32-bit, 16-bit... Hehehe.

But it should be noted that you cannot force it to be equal to 64 because your computer is 64-bit, that is to say `isize != i64`, you need to convert it in any case.

## array array

Arrays in Rust are represented as `[T;N]`. Among them, N represents the size of the array, and this size must be an integer value that can be obtained at compile time, and T represents the `generic` type, that is, any type. We can declare and use an array like this:

```rust
let a = [8, 9, 10];
let b: [u8;3] = [8, 6, 5];
print!("{}", a[0]);
```

Like Golang, the `N` (size) in Rust's array is also part of the type, ie `[u8; 3] != [u8; 4]`. This design is to use memory more safely and efficiently. Of course, this will bring a little difficulty to people who are exposed to similar concepts for the first time, such as the following code.

```rust
fn show(arr: [u8;3]) {
    for i in &arr {
        print!("{} ", i);
    }
}

fn main() {
    let a: [u8; 3] = [1, 2, 3];
    show(a);
    let b: [u8; 4] = [1, 2, 3, 4];
    show(b);
}
```

Compile and run it and you will get a compile error:

```
<anon>:11:10: 11:11 error: mismatched types:
 expected `[u8; 3]`,
    found `[u8; 4]`
(expected an array with a fixed size of 3 elements,
    found one with 4 elements) [E0308]
<anon>:11     show(b);
                   ^
<anon>:11:10: 11:11 help: see the detailed explanation for E0308
error: aborting due to previous error
```

This is because you are assigning an array of length 4 to a function that only expects an array of length 3 as an argument. So how to write a general show method to display arrays of any length? See the next section `Slice`

## Slice

Intuitively speaking, `Slice` is a slice of an `Array`, and through `Slice`, you can obtain access to part or all of an `Array`. Unlike `Array`, `Slice` can be dynamic, but its range cannot exceed the size of `Array`, which is different from Golang.

A `Slice` expression can be as follows: `&[T]` or `&mut [T]`.

The `&` symbol here is a difficult point. We might as well let go of this symbol and simply regard it as the turtle buttock of `Slice` - the rule. In addition, similarly, `Slice` can also access its elements through subscripts, and subscripts also start from 0.
You can declare and use a `Slice` like this:

```rust
let arr = [1, 2, 3, 4, 5, 6];
let slice_complete = &arr[..]; // Get all elements
let slice_middle = &arr[1..4]; // Get the middle element, and the finally obtained Slice is [2, 3, 4] . Slicing follows the left-closed-right-open principle.
let slice_right = &arr[1..]; // The final element obtained is [2, 3, 4, 5, 6] with a length of 5.
let slice_left = &arr[..3]; // The final obtained element is [1, 2, 3], and the length is 3.
```

How about it, I understand.
Then we use `Slice` to transform the above function

```rust
fn show(arr: &[u8]) {
    for i in arr {
        print!("{} ", i);
    }
    println!("");
}

fn main() {
    let a: [u8; 3] = [1, 2, 3];
    let slice_a = &a[..];
    show(slice_a);
    let b: [u8; 4] = [1, 2, 3, 4];
    show(&b[..]);
}
```
output
```
1 2 3
1 2 3 4
```

## dynamic array Vec

Students who are familiar with C++ STL may be familiar with C++ vector. Similarly, Rust also provides a similar thing. His name is `Vec`.

It seems inappropriate to talk about `Vec` in basic types, but it is widely used in practical applications, so let’s give a rough introduction first, and it will be described in detail in the chapter of collection types.

In Rust, `Vec` is represented as `Vec<T>`, where T is a generic type.

Here are some typical usages of `Vec`:

```rust
let mut v1: Vec<i32> = vec![1, 2, 3]; // declared by vec! macro
let v2 = vec![0; 10]; // Declare a dynamic array with an initial length of 10 and all values are 0
println!("{}", v1[0]); // Access array elements by subscript

for i in &v1 {
     print!("{}", i); // &Vec<i32> can be converted to &[i32] by Deref
}

println!("");

for i in &mut v1 {
     *i = *i+1;
     print!("{}", i); // variable access
}

```

Output result:

```
1
123
234
```

## The most native string str

You can use `str` to declare a string. In fact, in Rust, everything wrapped with `""` can be called `&str` (note the `&`, this is difficult, don’t worry about it, it’s not ?), but this type is rarely used alone, so we'll focus on the string type in the next section.

## Function Type Functions

A function is also a type. Here I will only popularize some basic concepts for you. The function type involves relatively high-level applications. I hope you can read it carefully in the following `Closure` chapter

Here is a small example

```rust
fn foo(x: i32) -> i32 { x+1 }

let x: fn(i32) -> i32 = foo;

assert_eq!(11, x(10));
```
