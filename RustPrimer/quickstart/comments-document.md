# Comments and Documentation

## comments
There are two types of comments in Rust, line comments and block comments. Its form is the same as C language.
The two annotations are:
> 1. Line comments are preceded by `//`. for example:

```
// I love Rust, but I hate Rustc.
```

> 2. Block comments use `/*` and `*/` to wrap the content that needs to be commented. for example:

```
/* W-Cat is a big fat cat, N-Cat is a highly myopic cat. */
```

## Documentation
Rust comes with annotations for documentation functions, namely `///` and `//!`. Support Markdown format
1. `///` is used to describe the item that follows it.
2. `//!` is used to describe the item that contains it, generally used in the header of the module file.
For example, enter the following in the main.rs file:

```
         //! # The first line
         //! The second line
         /// Adds one to the number given.
         ///
         /// # Examples
         ///
         /// ```
         /// let five = 5;
         ///
         /// assert_eq!(6, add_one(5));
         /// # fn add_one(x: i32) -> i32 {
         /// # x + 1
         /// # }
         /// ```
         fn add_one(x: i32) -> i32 {
             x + 1
         }
```

### Generate html document
* `rustdoc main.rs`

or

* `cargo doc`
