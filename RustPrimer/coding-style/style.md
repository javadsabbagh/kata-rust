# code style

## blank

* Each line cannot exceed 99 characters.
* Use only spaces for indentation, not TAB.
* Do not have whitespace at the end of lines and files.

### spaces

* Add spaces around binary operators, including equal signs in attributes:

```rust
#[deprecated = "Use `bar` instead."]
fn foo(a: usize, b: usize) -> usize {
    a + b
}
```

* Add spaces after semicolons and commas:

```rust
fn foo(a: Bar);

MyStruct { foo: 3, bar: 4 }

foo(bar, baz);
```

* Add spaces after the opening curly brace and before the closing curly brace of a single-line statement block or `struct` expression:

```rust
spawn(proc() { do_something(); })

Point { x: 0.1, y: 0.3 }
```

### Wrap lines

* For multiline function signatures, each newline is aligned with the first argument. Multiple arguments per line are allowed:

```rust
fn frobnicate(a: Bar, b: Bar,
              c: Bar, d: Bar)
              -> Bar {
    ...
}

fn foo<T: This,
       U: That>(
       a: Bar,
       b: Bar)
       -> Baz {
    ...
}
```

* Multi-line function calls generally follow the same rules as signatures. However, if the last parameter starts a statement block, the contents of the block can start on a new line, indented one level:

```rust
fn foo_bar(a: Bar, b: Bar,
           c: |Bar|) -> Bar {
    ...
}

// can be on the same line:
foo_bar(x, y, |z| { z. transpose(y) });

// Function bodies can also be indented on a new line:
foo_bar(x, y, |z| {
    z.quux();
    z.rotate(x)
})
```


### alignment

Common code doesn't have to be aligned with extra spaces in the line.


```rust
// good
struct Foo {
    short: f64,
    really_long: f64,
}

// bad
struct Bar {
    short: f64,
    really_long: f64,
}

// good
let a = 0;
let radius = 7;

// bad
let b = 0;
let diameter = 7;
```

### Avoid block comments

Use line comments:

```rust
// Wait for the main thread to return and set the process error code
// obviously.
```

instead of:

```rust
/*
 * Wait for the main thread to return and set the process error code
 * obviously.
 */
```

## Documentation comments

Doc comments are preceded by triple slashes (`///`) and indicate that you want the comment included in Rustdoc's output.
They support the [Markdown language](https://en.wikipedia.org/wiki/Markdown)
And is the primary way to annotate your public API.

Supported markdown features include all extensions listed in the [GitHub Flavored Markdown](https://help.github.com/articles/github-flavored-markdown) documentation, plus superscripts.

### Summary line

The first line in any documentation comment should be a short one-line sentence summarizing the code. This line is used for a short summary description in the Rustdoc output, so it's better to keep it short.

### Sentence Structure

All documentation comments, including the summary line, begin with a capital letter and end with a period, question mark, or exclamation point. It is better to use complete sentences rather than fragments.

The summary line should be written in [third-person singular declarative form](http://en.wikipedia.org/wiki/English_verbs#Third_person_singular_present).
Basically, this means using "Returns" instead of "Return".

For example:

```rust
/// According to the parameters provided by the compiler, set a default runtime configuration.
///
/// This function will block until the entire M:N scheduler pool exits.
/// This function also requires a local thread to be available.
///
/// # parameters
///
/// * `argc` and `argv` - Argument vectors. On Unix systems, this information is used by `os::args`.
///
/// * `main` - The initial process running in the M:N scheduler pool.
/// Once this process exits, the dispatch pool will start shutting down.
/// The entire pool (and this function) will only be executed after all child threads have finished executing.
///
/// # return value
///
/// The return value is used as the process return code. Success is 0, 101 is error.
```

### Avoid in-documentation comments

Inline doc comments _only_ comment crates and file-level modules:

```rust
//! Core library.
//!
//! The core library is...
```

### Interpret the context

Rust has no specific constructors, only functions that return new instances.
These are not visible in the auto-generated type documentation, so you should link to them specifically:

```rust
/// An iterator that yields `None` forever after the underlying iterator
/// yields `None` once.
///
/// These can be created through
/// [`iter.fuse()`](trait.Iterator.html#method.fuse).
pub struct Fuse<I> {
    //...
}
```

### The opening brace always appears on the same line.

```rust
fn foo() {
    ...
}

fn frobnicate(a: Bar, b: Bar,
              c: Bar, d: Bar)
              -> Bar {
    ...
}

trait Bar {
    fn baz(&self);
}

impl Bar for Baz {
    fn baz(&self) {
        ...
    }
}

frob(|x| {
    x.transpose()
})
```

### `match` branches have curly braces, unless they are single-line expressions.

```rust
match foo {
    bar => baz,
    quux => {
        do_something();
        do_something_else()
    }
}
```

### `return` statements have semicolons.

```rust
fn foo() {
    do_something();

    if condition() {
        return;
    }

    do_something_else();
}
```

### comma at end of line

```rust
Foo { bar: 0, baz: 1 }

Foo {
    bar: 0,
    baz: 1,
}

match a_thing {
    None => 0,
    Some(x) => 1,
}
```

### General Naming Conventions

In general, Rust prefers to use `CamelCase` for "type-level" constructs (types and traits) and `snake_case` for "value-level" constructs. A more precise convention:

| Items | Conventions |
| ---- | ---------- |
| Crates | `snake_case` (but prefer single words) |
| Modules | `snake_case` |
| Types | `CamelCase` |
| Traits | `CamelCase` |
| Enum variants | `CamelCase` |
| Functions | `snake_case` |
| Methods | `snake_case` |
| General constructors | `new` or `with_more_details` |
| Conversion constructors | `from_some_other_type` |
| Local variables | `snake_case` |
| Static variables | `SCREAMING_SNAKE_CASE` |
| Constant variables | `SCREAMING_SNAKE_CASE` |
| Type parameters | Concise `CamelCase`, usually a single capital letter: `T` |
| Lifetimes | Short lower case: `'a` |

<p>
In `CamelCase`, acronyms are treated as one word: use `Uuid` instead of
`UUID`. In `snake_case`, acronyms are all lowercase: `is_xid_start`.

In `snake_case` or `SCREAMING_SNAKE_CASE`, "words" should never contain only one letter,
Unless it's the last "word". So, we have `btree_map` instead of `b_tree_map`, and `PI_2` instead of `PI2`.

### Referencing the type in the function/method name

Function names often refer to type names, the most common examples of conventions are `as_slice`. If the type has a purely literal name (parameters are ignored),
Converting between type conventions and function conventions is straightforward:

typename | text in method
--------- | ---------------
`String`  | `string`
`Vec<T>`  | `vec`
`YourType`| `your_type`

Types involving tokens follow the following conventions. These rules overlap; the most applicable rule applies:

typename | text in method
--------- | ---------------
`&str`    | `str`
`&[T]`    | `slice`
`&mut [T]`| `mut_slice`
`&[u8]`   | `bytes`
`&T`      | `ref`
`&mut T`  | `mut`
`*const T`| `ptr`
`*mut T`  | `mut_ptr`

### Avoid redundant prefixes

The names of entries in a module should not be prefixed with the module name:

tend to

```rust
mod foo {
    pub struct Error { ... }
}
```

instead of

```rust
mod foo {
    pub struct FooError { ... }
}
```

This convention avoids stuttering (like `io::IoError`). Library clients can be renamed on import to avoid conflicts.

### Getter/setter methods

Some data types do not wish to provide direct access to their fields, but provide "getter" and "setter" methods for manipulating field state
(Often provide inspection or other functionality).

The convention for domain `foo: T` is:

* The method `foo(&self) -> &T` is used to get the current value of the field.
* The method `set_foo(&self, val: T)` is used to set the field. (The `val` parameter here may take `&T` or other types, depending on the context.)

Note that this convention is about getters/setters of data types in general, *not* about builder objects.

### Assertions

* Simple boolean assertions should be prefixed with `is_` or other short question word, e.g., `is_empty`.
* Common exceptions: `lt`, `gt`, and other recognized assertion names.

### import

A crate/module import should include the following parts, in order, separated by blank lines:

* `extern crate` directive
* External `use` imports
* Local `use` imports
* `pub use` import

For example:

```rust
// Crates.
extern crate getopts;
extern crate mylib;

// Standard library imports.
use getopts::{optopt, getopts};
use std::os;

// Import from a library we wrote.
use mylib::webserver;

// Will be re-exported when we import this module.
pub use self::types::Webdata;
```

### Avoid `use *` except in tests

Glob imports have several disadvantages:
* Harder to know where names are bound.
* They are forward incompatible because new upstream exports may conflict with existing names.

When writing the `test` submodule, it is appropriate to import `super::*` for convenience.

### Prefer full imports of types/traits when modules qualify functions.

For example:

```rust
use option::Option;
use mem;

let i: isize = mem::transmute(Option(0));
```

### Re-export the most important types at the crate level.

Crates `pub use` the most common types as a convenience, so clients don't have to remember or write the crate's module structure to use these types.

### Types and operations are defined together.

Type definitions and functions/modules that use them should be defined in the same module, with types appearing before functions/modules.
