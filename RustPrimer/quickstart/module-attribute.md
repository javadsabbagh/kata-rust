# Modules and attributes

Rust has two unique terms related to the module system: `crate` and `module`,
The crate has the same function as libary or package in other languages.
Each crate has a hidden root module under which a tree of submodules can be defined,
Its path uses `::` as a delimiter. A crate is made up of items, and multiple items are organized together through modules.

## define module

Define our module using the `mod` keyword:

```rust
// in src/lib.rs

mod chinese {
     mod greetings {

     }

     mod farewells {

     }
}

mod english {
     mod greetings {

     }

     mod farewells {

     }
}
```
Four submodules `chinese::{greetings, farewells}` and `english::{greetings, farewells}` are defined.
Modules are private by default, and can be made public using the `pub` keyword, and only public entries are allowed to be accessed outside the module.

A better organization in practice is to split a crate into multiple files:

```rust
// in src/lib.rs

pub mod chinese;

pub mod english;
```
These two statements tell Rust to look at `src/chinese.rs` and `src/english.rs`,
Or `src/chinese/mod.rs` and `src/english/mod.rs`.
First add some functions:

```rust
// in src/chinese/greetings.rs

pub fn hello() -> String {
     "Hello!".to_string()
}
```

```rust
// in src/chinese/farewells.rs

pub fn goodbye() -> String {
     "Goodbye!".to_string()
}
```

```rust
// in src/english/greetings.rs

pub fn hello() -> String {
     "Hello!".to_string()
}
```

```rust
// in src/english/farewells.rs

pub fn goodbye() -> String {
     "Goodbye!".to_string()
}
```
The function is also private by default, for later use we need the `pub` keyword to make it public.

## import crates

In order to use the crate named `phrases` we created earlier, we need to first declare the import

```rust
// in src/main.rs

extern crate phrases;

fn main() {
     println!("Hello in Chinese: {}", phrases::chinese::greetings::hello());
}
```

Rust also has a `use` keyword that allows us to import crate items into the current scope:

```rust
// in src/main.rs

extern crate phrases;

use phrases::chinese::greetings;
use phrases::chinese::farewells::goodbye;

fn main() {
     println!("Hello in Chinese: {}", greetings::hello());
     println!("Goodbye in Chinese: {}", goodbye());
}
```
However, we do not recommend importing functions directly, which is more likely to cause namespace conflicts, and it is better to only import modules.
If you want to import multiple items from the same module, you can use the brace shorthand:

```rust
use phrases::chinese::{greetings, farewells};
```
If you are importing all, you can use the wildcard `*`. Renaming can be done using the `as` keyword:

```rust
use phrases::chinese::greetings as chinese_greetings;
```

Sometimes we need to import functions from the outer crate into another module,
At this time, `pub use` can be used to provide an extension interface without mapping the code hierarchy.
for example

```rust
// in src/english/mod.rs

pub use self::greetings::hello;
pub use self::farewells::goodbye;

mod greetings;

mod farewells;
```
The `pub use` statement brings the function into the current module,
So that we now have `phrases::english::hello()` function and `phrases::english::goodbye()` function,
even though they are defined in `phrases::english::greetings::hello()`
and `phrases::english::farewells::goodbye()`,
The organization of the internal code does not reflect our extension interface.

By default, `use` declarations represent absolute paths from the root crate.
In addition, we can use `use self::` to express the position relative to the current module,
`use super::` indicates the upper level of the current location, and the path prefixed with `::` indicates the root crate path.

```rust
use foo::baz::foobaz; // foo is at the root of the crate

mod foo {
     use foo::bar::foobar; // foo is at crate root
     use self::baz::foobaz; // self refers to module 'foo'

     pub mod bar {
         pub fn foobar() { }
     }

     pub mod baz {
         use super::bar::foobar; // super refers to module 'foo'
         pub fn foobaz() { }
     }
}
```

## Attributes

In Rust, an attribute is metadata applied to a crate, module, or item,
Mainly used:

* Implement conditional compilation
* Set crate name, version and type
* Cancel warnings for suspicious code
* Set compiler options
* link external library
* mark test function

Attributes can be used in two ways: `#![crate_attribute]` applies to the entire crate,
And `#[crate_attribute]` applies to the next module or entry.
Attribute parameters also come in three different forms:

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

Here are a few commonly used attributes:

* `#[path="foo.rs"]` is used to set the file path that a module needs to load.
* `#[allow(dead_code)]` is used to suppress the default lint checks for dead code.
* `#[derive(PartialEq, Clone)]` is used to automatically derive the implementation of the two features `PartialEq` and `Clone`.

