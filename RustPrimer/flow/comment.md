# comment

In Rust code files, we can usually see 3 kinds of comments.

- line comment
- Documentation comments
- Module comments

## Line comments

  After `//`, until the end of the line, it is a comment and will not affect the behavior of the program.

```rust
// create a binding
let x = 5;

let y = 6; // create another binding
```

## Documentation comments

Documentation comments use ```///```, which are generally used for descriptions of functions or structures (fields), placed above the object to be described. The markup syntax in markdown format can be used inside the documentation comments, which can be used for automatic documentation extraction of the rustdoc tool.

    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, add_one(5));
    /// # fn add_one(x: i32) -> i32 {
    /// #     x + 1
    /// # }
    /// ```
    fn add_one(x: i32) -> i32 {
        x + 1
    }


## Module comments

Module comments use ```//!``` to describe the function of this module. Usually placed at the head of the module file.

```rust
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
```

PS: Relative to `///`, `//!` is used to comment the item that contains it (that is, crate, module or function), not the item after it.


## Others: Compatible with C language comments

Rust also supports C-compatible block comment writing: `/* */`. But it is not recommended, please try not to use this comment style (will be despised).

```rust
/*
    let x = 42;
    println!("{}", x);
*/
```
