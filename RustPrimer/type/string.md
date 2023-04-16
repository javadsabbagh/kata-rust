# String

In this chapter, we will focus on strings.

Students who have just learned Rust may be confused by Rust strings, such as `str`, `String`, `OsStr`, `CStr`, `CString`, etc...
In fact, if you don't do FFI, the commonly used string types are only the first two. Let's focus on the first two strings of Rust.

What you need to understand is that a string in Rust is actually an array of bytes encoded as UTF-8. It's a mouthful to say so. Simply put, Rust strings store a u8 array internally, but this array is obtained by encoding Unicode characters through UTF-8. Therefore, it can be seen that Rust natively supports the Unicode character set (Python2 code farmers burst into tears).

##str

First of all, let's take a look at `str`. Literally, Rust's string is expressed as: `&'static str` (it doesn't matter if you don't understand this expression, & means you know the reference, and static means static, you know , Okay, it’s all set), that is, all the strings wrapped with `""` that you write in the code are declared as immutable and static strings. And our following statement:

```rust
let x = "Hello";
let x:&'static str = "Hello";
```

In fact, a reference to the static variable `"Hello"` is passed to `x`. At the same time, the string here is immutable!

Strings also support escape characters:
For example as follows:

```rust
let z = "foo
bar";
let w = "foo\nbar";
assert_eq!(z, w);
```

You can also add `r` before the string literal to avoid escaping

     // no escape sequence
     let d: &'static str = r"abc \n abc";
     //Equivalent to
     let c: &'static str = "abc \\n abc";

##String

Just having `str` is really not enough. After all, what we want more in practical applications is a variable, variable-length string. At this time, a string `String` declared on the heap was designed.
It can grow or shrink dynamically, so how to declare it? Let's first introduce a simple way to convert from `str`:

```rust
let x:&'static str = "hello";

let mut y:String = x.to_string();
println!("{}", y);
y.push_str(", world");
println!("{}", y);
```

I know you will ask:——
     So how to turn a `String` back into `&str`?
     Answer: Use the `&*` symbol

```rust
fn use_str(s: &str) {
    println!("I am: {}", s);
}

fn main() {
    let s = "Hello".to_string();
    use_str(&*s);
}
```

Let's analyze it. The following part will involve some knowledge of `Deref`. You may need to preview it. If you don't understand it, you can skip the next paragraph:

First of all, `&*` is a combination of two symbols `&` and `*`. According to the order of operations of Rust, `Deref` is performed on `String` first, that is, `*` operation.

Since `String` implements `impl Deref<Target=str> for String`, which is equivalent to an operator overload, you can get a `str` type through `*`. But we know that a single `str` cannot exist directly in Rust, so we need to perform the `&` operation on it first to get the result of `&str`.

Someone said, I found that as long as the operator `&` is used, the above compilation will pass.
This is actually a compiler fault, because the Rust compiler will insert enough `*` after `&` to satisfy the `Deref` feature as much as possible. This feature will fail in some cases, so in order to save yourself trouble, it is better to write all operators.


What you need to know is that converting `String` to `&str` is very painless and has almost no overhead. But conversely, converting `&str` to `String` needs to request memory on the heap, so be careful.

We can also convert a UTF-8 encoded byte array into a String, such as

```rust
// some bytes stored in Vec
let miao = vec![229,150,181];

// We know that these bytes are legal UTF-8 encoded strings, so directly unwrap()
let meow = String::from_utf8(miao).unwrap();

assert_eq!("喵", meow);
```

## index access

Some people will equate the strings in Rust with their idiomatic strings, so the following code appears

```rust
let x = "hello".to_string();
x[1]; //Compile error!
```

Rust's string actually does not support subscript access, but we can access it by converting it into an array

```rust
let x = "Ouch, I'm going".to_string();
for i in x.as_bytes() {
    print!("{} ", i);
}

println!("");

for i in x.chars() {
    print!("{}", i);
}

x.chars().nth(2);
```

## String Slicing

Slicing strings is a very dangerous thing to do, and while Rust supports it, I don't recommend it. Because Rust's string slice is actually sliced bytes. This also caused a serious consequence. If the position of your slice happens to be inside a Unicode character, Rust will panic at Runtime, causing the entire program to crash.
Because this operation is so dangerous, I won't demonstrate it...
