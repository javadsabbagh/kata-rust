# Operators and format strings

The current Rust information, neither the Book nor the RustByExample has a unified and complete introduction to Rust operators. A very important reason is that most of Rust's operation symbols and C++ are exactly the same.

## unary operator

As the name implies, a unary operator is an operator that specifically manipulates a Rust element, mainly including the following:

* `-`: Negative, specially used for numeric types.
* `*`: Dereference. This is a useful symbol, closely related to the `Deref` (`DerefMut`) trait.
* `!`: Negate. I believe everyone is familiar with the negation operation, so I won’t say much. Interestingly, when this operator is used on numeric types, it inverts every bit of it! In other words, if you do `!` to a `1u8`, you will get a `254u8`.
* `&` and `&mut`: lease, borrow. Lease its use right from an owner, which means renting a read-only use right and a read-write use right respectively.

## Binary operators

### Arithmetic operators

Arithmetic operators have corresponding traits, and they are all under `std::ops`:

* `+`: addition. Implemented `std::ops::Add`.
* `-`: Subtraction. Implemented `std::ops::Sub`.
* `*`: Multiplication. Implemented `std::ops::Mul`.
* `/`: Division. Implemented `std::ops::Div`.
* `%`: take the remainder. Implemented `std::ops::Rem`.

### bitwise operators

Similar to arithmetic operators, bitwise operations have corresponding traits.

* `&`: AND operation. Implemented `std::ops::BitAnd`.
* `|`: OR operation. Implemented `std::ops::BitOr`.
* `^`: XOR. Implemented `std::ops::BitXor`.
* `<<`: left shift operator. Implemented `std::ops::Shl`.
* `>>`: Right shift operator. Implemented `std::ops::Shr`.

### Lazy boolean operator

There are three logical operators, namely `&&`, `||`, `!`. The first two of them are called lazy boolean operators, which is why they are called this name. It is because the logical short-circuit problem of other C-like languages ​​also occurs in Rust. So I took such a tall and natural name.
Its function is the same as that in C language! Oh, by the way, the difference is that this operator in Rust can only be used on bool type variables. Expressions like `1 && 1` are killing me.

### Comparison Operators

Comparison operators are actually syntactic sugar for certain traits. The difference is that the traits implemented by comparison operators are only two `std::cmp::PartialEq` and `std::cmp::PartialOrd`

Among them, `==` and `!=` implement `PartialEq`.
However, `<`, `>`, `>=`, `<=` implement `PartialOrd`.

Students who open the standard library (good habit, encouragement) while reading this section will be surprised to find that, no, there are four traits under the `std::cmp` mod, and it is more logical from the naked eye Wouldn't `Ord` and `Eq` be better? In fact, Rust's handling of these four traits is very clear. The divergence mainly exists in floating-point types.
Students who are familiar with IEEE must know that floating-point numbers have a special value called `NaN`, which means an undefined floating-point number. In Rust, you can use `0.0f32 / 0.0f32` to find its value. Then the problem comes, this number is a certain value, but it represents an uncertain number! So what is the result of `NaN != NaN`? The standard tells us that is `true`. But this writing does not conform to the definition of `total equal` (every digit is the same and two numbers are the same) in the definition of `Eq`. So with the definition of `PartialEq`, we only support partial equality. Well, I will specifically refer to it in the case of NaN.

For universal use, the Rust compiler chooses `PartialOrd` and `PartialEq` as its default comparison symbol traits. We just need to be consistent with the central government.

## Type Conversion Operators

In fact, this is not an operator, because it is the word `as`.

This is the explicit type conversion that everyone is familiar with in C language.

show u the code:

```rust
fn avg(vals: &[f64]) -> f64 {
    let sum: f64 = sum(vals);
    let num: f64 = len(vals) as f64;
    sum / num
}
```

## overloaded operator

There are many traits mentioned above. Some people will ask, why do you say so much?

Answer, for operator overloading!

Rust supports operator overloading (a coffee language fainted in the toilet).

Regarding this part, there will be a very detailed description in Section 30 of this book, so I will not talk about it here. The last chestnut is for everyone, just for reference:

```rust
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
struct A(i32);

impl Add for A {
    type Output = A;
    fn add(self, rhs: A) -> A {
        A(self.0 + rhs.0)
    }
}

impl Sub for A {
    type Output = A;
    fn sub(self, rhs: A) -> A{
        A(self.0 - rhs.0)
    }
}

fn main() {
    let a1 = A(10i32);
    let a2 = A(5i32);
    let a3 = a1 + a2;
    println!("{}", (a3).0);
    let a4 = a1 - a2;
    println!("{}", (a4).0);
}
```

output:

```
15
5
```

# format string

Speaking of formatting strings, Rust adopts a usage similar to format in Python. Its core components are five macros and two traits: `format!`, `format_arg!`, `print!`, `println!`, `write!`; `Debug`, `Display`.

I believe that you used the two macros `print!` or `println!` when writing the Rust version of Hello World, but in fact the most important thing is `format!`. The first two macros just convert `format!` The result is output to the console.

So, let's explore the magic macro `format!`.

Here, it is useless to enumerate the definition of `format!`, because it is too complicated. I just introduce a few typical usages for you. If you learn it, you can basically cover 80% of your usual needs.

First, let's analyze a typical call of format

```rust
fn main() {
     let s = format!("{1} is a big fat man with {0:>0width$}KG weight and {height:?}cm height",
                     81, "wayslog", width=4, height=178);
     // I was forced to sacrifice myself...
     print!("{}", s);
}
```

We can see that when the `format!` macro is called, the parameters can be of any type, and the position parameter and the key-value parameter can be mixed. But one thing to note is that the key-value value can only appear after the position value and does not occupy the position. For example, in the example you use `3$` to refer to `width`, it is definitely not `width`, but an error will be reported.
There is a little rule about parameters here, that is, the parameter type must implement certain traits under the `std::fmt` mod. For example, we see that most of the native types implement the two macros `Display` and `Debug`, and the integer type also implements `Binary`, and so on.

Of course, we can use `{:type}` to call these parameters.

For example:

```rust
format!("{:b}", 2);
// call `Binary` trait
// Get : 10
format!("{:?}", "Hello");
// call `Debug`
// Get : "Hello"
```

Also please remember: If the type field is empty, the `Display` trait will be called by default.

In fact, there are more formulas for the things behind the `:`, let's analyze it from the above `{0:>0width$}`.

First of all, `>` is a semantic meaning, which means that the generated string is aligned to the right, so we get the value of `0081`. There are also `<` (align left) and `^` (center).

The next `0` is a special filling syntax, which means to fill up the vacancy of the number with 0. It should be noted that when 0 acts on a negative number, for example, the weight of the wayslog in the above example is -81, then you In the end, you will get `-0081`; of course, if you don’t write anything, you will fill it with spaces; in this bit, there will also be `+`, `#` syntax, which is quite strange to use, and it is usually not used .

Finally, there is a combined expression `width$`. Here, you can quickly recognize that it means `width=4` in the following key-value value pair. You guessed it right, this value represents the length of the string after formatting. It can be an exact length value, or a string ending with `$`, and the part before `$` can be written as a key or a postion.

Finally, what you need to remember is that there will be an area called precision between width and type (it can be omitted and not written as an example), and their representations usually start with `.`, such as `.4` Four digits of precision after the decimal point. The most disturbing thing is that you can still refer to parameters at this position, you only need to use `.N$` to represent a position parameter like the width above, but you cannot refer to the key-value type. This bit has a special usage, that is `.*`, it does not represent one value, but two values! The first value represents the exact number of bits, and the second value represents the value itself. This is an awkward usage, and extremely easy to match other parameters. Therefore, I suggest that you try to write the formatting expressions clearly and clearly in a standard form when you have the ability or time. Especially when faced with a complex format string.

Okay, after talking so much, I guess you are also dizzy, let me write the complete usage of the format macro. Carefully understand and refine the meaning and position of each word.

```
format_string := <text> [ format <text> ] *
format := '{' [ argument ] [ ':' format_spec ] '}'
argument := integer | identifier

format_spec := [[fill]align][sign]['#'][0][width]['.' precision][type]
fill := character
align := '<' | '^' | '>'
sign := '+' | '-'
width := count
precision := count | '*'
type := identifier | ''
count := parameter | integer
parameter := integer '$'
```

Finally, leave homework.
Given the parameter list as follows:
`(500.0, 12, "ELTON", "QB", 4, CaiNiao="Mike")`

Please write a format string that can output a sentence at the end and the parameters *all* have been used *at least once*, and play it yourself to experiment.

```
Tang Mike from the rust.cc community has a full eye degree of 0500.0 degrees but still earns 100 QBs by working hard every day.
But ELTON only needs 12 hours of sleep to marry Bai Fumei.
```
