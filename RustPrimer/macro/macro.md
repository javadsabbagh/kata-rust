#Macro

## Introduction

Anyone who has learned C language knows that `#define` is used to define macros (macro), and many teachers in universities tell you to use macros as little as possible, because macros in C are a very dangerous thing-macros are just simple text Replacement, regardless of syntax and type, is very error-prone. People who have heard or used Lisp think that macros are extremely powerful. Even Paul Gram, the founder of the largest startup incubator company in the United States, strongly advocates how powerful Lisp's macros are. So what exactly is a macro? This chapter takes you through Rust's macro system to uncover the mystery of macros.

Macros in Rust are almost everywhere. In fact, the first Rust program you write has already used macros. Yes, it is the famous hello-world. `println!("Hello, world!")` This sentence looks like a function call, but an exclamation point is added after the "function name", which is specially used to distinguish between ordinary function calls and macro calls. In addition, in terms of form, another difference from function calls is that parameters can be enclosed in any of parentheses (`()`), curly braces (`{}`), and square brackets (`[]`) , for example, this line can also be written as `println!["Hello, world!"]` or `println!{"Hello, world!"}`, but there are customary brackets for Rust's built-in macros, such as `vec!` Use square brackets, `assert_eq!` use parentheses.

Since macros look so much like ordinary functions, what are the benefits of using macros? Can functions be used instead of macros? The answer is obviously no. First, Rust functions cannot accept any number of parameters. Second, functions cannot operate on grammatical units, that is, operate on grammatical elements as parameters to generate code, such as `mod`, `crate` These are Rust It is impossible to directly use functions to operate these built-in keywords, and macros have this ability.

Compared with functions, macros are used to generate code. Where macros are called, the compiler will first expand the macros to generate code, and then compile the expanded code.

The macro definition format is: `macro_rules! macro_name { macro_body }`, where `macro_body` is very similar to pattern matching, `pattern => do_something`, so Rust's macro is also called Macro by example (example-based macro). Among them, `pattern` and `do_something` are enclosed in paired brackets, and the brackets can be any one of parentheses, square brackets, and curly brackets. A match can have multiple branches, each ending with a semicolon.

Let's give a simple example first

```rust
macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            println!("function {:?} is called", stringify!($func_name))
        }
    )
}

fn main() {
    create_function!(foo);
	foo();
}

```

The simple example above is used to create a function. The generated function can be called like a normal function, and this function can print its own name. When the compiler sees `create_function!(foo)`, it will look for a macro definition called `create_function` from the front. After finding it, it will try to substitute the parameter `foo` into `macro_body`, and execute each pattern in order Matching, as long as there is a match, the parameters defined on the left side of `=>` will be substituted into the right side for replacement. If the replacement is unsuccessful, the compiler will report an error and will not continue to match. If the replacement is successful, the right side will be replaced The code of the macro is placed where the macro is called. In this example, there is only one pattern, namely `$func_name:ident`, which means to match an identifier. If it matches, assign this identifier to `$func_name`. The variables in the macro definition all start with `$`. The corresponding types are also separated by colons, where `ident` is the type of the variable `$func_name`, which means that this variable is an `identifier`, which is a grammatical type (designator), and common types such as `char, &str , i32, f64` These are types at the semantic level. The parameter `foo` passed to the macro call `create_function` in the `main` function is just an identifier (`ident`), so it can be matched, `$func_name` is equal to `foo`, and then `$func_name` Substituting the value of `=>` to the right, it becomes the following

```rust
fn foo() {
    println!("function {:?} is called", stringify!(foo))
}
```

So in the end the actual code compiled by the compiler is

```rust
fn main() {
    fn foo() {
	    println!("function {:?} is called", stringify!(foo))
	}
	foo();
}
```

After the `create_function` macro is defined above, it can be used to generate functions at will. For example, calling `create_function!(bar)` will get a function named `bar`

Through the above example, everyone should have a general understanding of macros. Let's talk about the various components of the macro in detail.



## The structure of the macro

### macro name

The parsing of macro names is slightly different from that of functions. The definition of a macro must appear before the macro call, which is similar to the function in C - the function definition or declaration must be before the function call, but the Rust macro does not have a simple declaration, so the macro It needs to be defined before the call, while Rust functions can be defined after the function call. The sequence dependency of macro calling and macro definition includes macros imported from other modules, so be careful when importing macros from other modules, which will be discussed in detail later.

In the following example, the macro definition is behind the macro call. The compiler will report an error saying that the macro definition cannot be found, but the function is fine.

```rust
fn main() {
    let a = 42;
    foo(a);
	bar!(a);
}

fn foo(x: i32) {
	println!("The argument you passed to function is {}", x);
}

macro_rules! bar {
	($x:ident) => { println!("The argument you passed to macro is {}", $x); }
}
```

In the above example, move the macro definition before the `main` function or above the `bar!(a)` call in the `main` function, then it can compile and run normally.

Although a macro call is very similar to a function call, the name of the macro and the name of the function are in different namespaces. The reason why it is proposed is that in some programming languages, macros and functions are under the same namespace. See the following example will understand

```rust
fn foo(x: i32) -> i32 {
    x * x
}

macro_rules! foo {
    ($x:ident) => { println!("{:?}", $x); }
}
fn main() {
    let a = 5;
	foo!(a);
    println!("{}", foo(a));
}
```

### designator

The variables in the macro all start with `$`, and the rest are matched literally. The variables starting with `$` are used to represent syntactic elements. In order to limit what type of syntactic elements to match, you need Qualify with a designator, just like ordinary variable binding, use a colon to separate the variable and type. The current macro supports the following designators:

* ident: identifier, used to represent the function or variable name
* expr: expression
* block: code block, multiple statements wrapped in curly braces
* pat: pattern, the pattern in the normal pattern matching (not the pattern of the macro itself), such as `Some(t)`, `(3, 'a', _)`
* path: path, note that this is not the file path in the operating system, but the qualified name separated by double colons, such as `std::cmp::PartialOrd`
* tt: single syntax tree
* ty: type, semantic type, such as `i32`, `char`
* item: item,
* meta: meta entry
* stmt: single statement, such as `let a = 42;`

After adding these type restrictions, the macro will not match aimlessly when performing matching. For example, expressions are not allowed where identifiers are required, otherwise the compiler will report an error. The macros in the C/C++ language are just simple text replacements without grammatical considerations, so they are very error-prone.

### repetition

A big difference between macros and functions is that macros can accept any number of parameters, such as `println!` and `vec!`. How is this possible?

That's right, repetition (repetition). The repetition of the pattern is not controlled by the loop (for/while) in the program, but two special symbols `+` and `*` are specified, which are similar to regular expressions, because regular expressions do not care about specific matching objects Is it the name of a person or a country. As with regular expressions, `+` means one or more times (at least one), and `*` means zero or more times. Repeated patterns need to be enclosed in parentheses and `$` outside, such as `$(...)*`, `$(...)+`. It should be noted that the brackets here, like other places in the macro, can be any of the three types of brackets, because the brackets are only used to mark the beginning and end of a pattern. In most cases, repeated patterns are separated by commas or semicolon separated, so you will often see `$(...),*`, `$(...);*`, `$(...),+`, `$(... );+` are used to indicate repetition.

Let's look at an example

```rust
macro_rules! vector {
	($($x:expr),*) => {
		{
			let mut temp_vec = Vec::new();
			$(temp_vec.push($x);)*
			temp_vec
		}
	};
}

fn main() {
	let a = vector![1, 2, 4, 8];
	println!("{:?}", a);
}
```

This example looks complicated at first, so let's analyze it.

First look at the left side of `=>`, the outermost layer is parentheses. As mentioned earlier, the parentheses can be any of parentheses, square brackets, and curly brackets, as long as they are matched. Then look at the `$(...),*` in the brackets, which is the repeating pattern just mentioned. The repeating pattern is separated by commas, and the repeating content is `$x:expr`, that is, it can match zero or more times Multiple expressions separated by commas, such as `vector![]` and `vector![3, x*x, s-t]` can be matched successfully.

Then look at the right side of `=>`, the outermost layer is also a parenthesis, and a semicolon at the end indicates the end of this branch. Inside is a code block surrounded by curly braces. There is no semicolon in the last line, indicating that the value of this macro is an expression, and `temp_vec` is returned as the value of the expression. The first statement is to use `Vec::new()` to generate an empty vector, and then bind it to the variable variable `temp_vec`. The second sentence is more special, similar to the left side of `=>`, also used to represent repeated patterns, and it corresponds to the left side one by one, that is, the left side matches an expression (`expr`), and the matched expression will be used in `temp_vec.push($x);` inside, so the `vector![3, x*x, s-t]` call expands to

```rust
{
	let mut temp_vec = Vec::new();
	temp_vec.push(3);
	temp_vec.push(x*x);
	temp_vec.push(s-t);
	temp_vec
}
```

Looking at a very complicated macro, it is very simple to analyze it in detail, don't be disturbed by these symbols

### recursion

In addition to repetition, macros also support recursion, that is, calling themselves when the macro is defined, similar to a recursive function. Because rust's macro itself is a kind of pattern matching, and recursion in pattern matching is the most common way of writing in functional languages. Those with functional programming experience should be familiar with this. Let's look at a simple example:

```rust
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
```

Because pattern matching is matched in the order of branches, once the match is successful, it will not be matched further down (even if it can be matched later), so the recursion in pattern matching is the simplest case written in the first branch. The more cases are included below. The same is true here, the first branch `($x:expr)` only matches one expression, the second branch matches two or more expressions, note that the plus sign means match one or more, and then inside it is used `min` in the standard library compares the size of two numbers, the first expression and the smallest of the remaining expressions, where the smallest of the remaining expressions is a recursive call to the `find_min!` macro, just like the recursive function, Each recursion is matched from top to bottom, only to the base case. Let's write `find_min!(5u32, 2u32 * 3, 4u32)` macro expansion process

1. `std::cmp::min(5u32, find_min!(2u32 * 3, 4u32))`
2. `std::cmp::min(5u32, std::cmp::min(2u32 * 3, find_min!(4u32)))`
3. `std::cmp::min(5u32, std::cmp::min(2u32 * 3, 4u32))`

It is as simple to analyze as a recursive function.

### Hygiene (hygienic Macro)

With repetition and recursion, the combination is a very powerful weapon, which can solve many things that cannot be abstracted by ordinary functions. But there will be a security problem, and it is also the most error-prone place for macros in C/C++. However, Rust introduces hygienic (Hygiene) macros like Scheme, which effectively avoids such problems.

The macro in C/C++ is just a simple text replacement. After the following C is preprocessed by the macro, the variable `a` defined outside the macro will be mixed with the variable defined inside, so that it is defined in the outer layer of the scope shadow. It will lead to some very weird problems. If you don't look at the specific definition of the macro and analyze it carefully, it is difficult to find such bugs. Such macros are unhygienic, but some weird Hackers think this is a great feature. For example, the macros in the CommanLisp language are very powerful, but they are not hygienic. Some Hackers are proud of this and do some The tricks and tricks deliberately create such a shadow behavior to achieve some very fancy effects. I don’t want to make too many comments here. Students who are more familiar with C can analyze whether the running result of the following code is the same as the first impression.

```c
#define INCI(i) {int a=0; ++i;}
int main(void)
{
    int a = 0, b = 0;
    INCI(a);
    INCI(b);
    printf("a is now %d, b is now %d\n", a, b);
    return 0;
}
```

Hygienic macros were first introduced by the Scheme language, and later many languages basically adopted hygienic macros, that is, the compiler or runtime will ensure that the variables or functions defined in the macro will not conflict with those outside, and defined in the macro in a normal way Variable scope does not go outside the macro.

```rust
macro_rules! foo {
    () => (let x = 3);
}

macro_rules! bar {
    ($v:ident) => (let $v = 3);
}

fn main() {
    foo!();
    println!("{}", x);
	bar!(a);
	println!("{}", a);
}
```

The variable `x` in the macro `foo!` in the above code is defined in a normal way, so its scope is limited to the macro, and the compiler will report an error if it refers to `x` after the macro call is completed. To make the variables defined in the macro still valid after the macro call ends, it needs to be defined as in `bar!`. However, the rules for `item` are somewhat different. For example, after a function is defined in a macro in a normal way, the function is still available after the macro is called, and the following code can be compiled normally.

```rust
macro_rules! foo {
    () => (fn x() { });
}

fn main() {
    foo!();
    x();
}
```

## Import and export (import/export)

As mentioned above, macro names are parsed in order, so importing macros from other modules is different from importing functions and traits. Macro import and export use `#[macro_use]` and `#[macro_export]`. The macros defined in the parent module are visible to the submodules under it. To make the macros defined in the submodule available in the parent module behind it, you need to use `#[macro_use]`.

```rust
macro_rules! m1 { () => (()) }

// macro m1 is available here

mod foo {
     // macro m1 is available here

     #[macro_export]
     macro_rules! m2 { () => (()) }

     // macros m1 and m2 are available here
}

// macro m1 is available here
#[macro_export]
macro_rules! m3 { () => (()) }

// macros m1 and m3 are available here

#[macro_use]
mod bar {
     // macros m1 and m3 are available here

    macro_rules! m4 { () => (()) }

     // macros m1, m3, m4 are available here
}

// Macros m1, m3, m4 are all available
```

Only macros marked `#[macro_export]` between crates can be imported by other crates. Assuming the above example is part of the code in the `foo` crate, only `m2` and `m3` can be imported by other crates. The way to import is to add `#[macro_use]` before `extern crate foo;`

```rust
#[macro_use]
extern crate foo;
// m2, m3 are both imported in foo
```

If you only want to import a macro in the `foo` crate, such as `m3`, add parameters to `#[macro_use]`
```rust
#[macro_use(m3)]
extern crate foo;
// only m3 is imported in foo
```

## debug

Although the macro function is very powerful, it is more difficult to debug than ordinary code, because the hints given by the compiler by default are all after macro expansion, not the original program you wrote. It is more difficult to establish a connection between them, because it requires your brain to be able to human-human compile and expand macro code. Fortunately, the compiler provides us with the `--pretty=expanded` option, which allows us to see the expanded code. Through this expanded code, there is a direct correspondence with the original program you wrote yourself. The relationship, and the error given by the compiler is also a direct correspondence.

At present, you need to use the unstable option to expand the macro. You can view the code after macro expansion through `rustc -Z unstable-options --pretty=expanded hello.rs`. If you are using cargo, you can use `cargo rustc -- -Z unstable- options --pretty=expanded` will expand the macros in the project. However, it is currently impossible to expand only part of the macro, and due to hygiene, some special processing (mangle) will be done to the names in the macro, so the readability of the code is relatively poor after all the macros in the program are expanded, but it is still It is more reliable than relying on the brain to expand.

Let's take a look at the expansion result of `println!("Hello, world!")` in the simplest hello-word program. For hygiene, the internal temporary variable is named `__STATIC_FMTSTR` to avoid name conflicts, even if it is simple It still doesn't look so intuitive after the sentence is expanded, so I won't analyze it in detail here.

```
$ rustc -Z unstable-options --pretty expanded hello.rs
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
fn main() {
    ::std::io::_print(::std::fmt::Arguments::new_v1({
                                                        static __STATIC_FMTSTR:
                                                               &'static [&'static str]
                                                               =
                                                            &["Hello, world!\n"];
                                                        __STATIC_FMTSTR
                                                    },
                                                    &match () { () => [], }));
}
```
