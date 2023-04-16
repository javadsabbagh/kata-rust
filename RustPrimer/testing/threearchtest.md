# test

> Program testing is an effective way to find bugs, but it does nothing to prove the absence of bugs.
>
> Edsger W. Dijkstra, "The Humble Programmer" (1972)

As an important part of the software engineering quality assurance system, testing is something that should attract our full attention and attention. As mentioned earlier, the design of the Rust language integrates a large number of best engineering practices summed up in the past ten years, and the native integration of testing also reflects this. Let's see how Rust designs test features.

Rust's testing features are divided into three levels according to the granularity:

1. Function level;
2. Module level;
3. Engineering grade;

Additionally, Rust supports testing against documentation.

## Function level tests

In this chapter, we use the practice of creating a library to explain the content of testing. We first use cargo to create a library project: `adder`

```
$ cargo new adder
$ cd adder
```

### `#[test]` flag
Open the `src/lib.rs` file, you can see the following code

```rust
#[test]
fn it_works() {
    // do test work
}
```

In Rust, you only need to add `#[test]` above a function to indicate that it is a function for testing.

With this attribute, these functions are ignored when compiling with `cargo build`. These functions can be run with `cargo test`. Similar to the following effect:

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Rust provides two macros to perform test assertions:

```rust
assert!(expr) Tests whether an expression is true or false
assert_eq!(expr, expr) Tests whether the results of two expressions are equal
```
for example

```rust
#[test]
fn it_works() {
    assert!(false);
}
```

Run `cargo test`, you will get a prompt similar to the following

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works... FAILED

failures:

---- it_works stdout ----
        thread 'it_works' panicked at 'assertion failed: false', /home/steve/tmp/adder/src/lib.rs:3



failures:
    it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /home/steve/src/rust/src/libtest/lib.rs:247
```

### `#[should_panic]` flag

If your test function is not completed, or has not been updated, or intentionally crashes it, but in order to make the test complete smoothly, we can proactively add `#[should_panic]` to the test function, so that `cargo test will not be allowed ` An error was reported.

like

```rust
#[test]
#[should_panic]
fn it_works() {
    assert!(false);
}
```

Running `cargo test`, the result is similar to the following:

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

### `#[ignore]` flag

Sometimes, a test function is very time-consuming, or has not been updated for a while, and we don't want it to participate in the test, but we don't want to delete it. At this time, `#[ignore]` comes in handy.

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

By writing this, the function will not be tested when running `cargo test`.

## Module level tests

Sometimes, we will organize a batch of test cases. At this time, a modular organizational structure helps to establish a structured test system. In Rust, it can be written like this:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::add_two;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

That is, write `#[cfg(test)]` on top of `mod`, indicating that this module is a test module. A test module can contain several test functions, and the test module can also continue to contain test modules, that is, the nesting of modules.

In this way, a structured test system is formed, which is very convenient.


## Engineering level test

For function-level and module-level tests, the code is written in the same file as the module (compilation unit) to be tested, and white-box testing is generally done. Engineering-level testing is generally done with black-box integration testing.

Let's look at the directory of a project. In this directory, there is a `tests` folder (if not, create it manually)

```
Cargo.toml
Cargo.lock
examples
src
tests
```

We create a file `testit.rs` in the tests directory, and the name can be whatever you want. The content is:

```rust
extern crate adder;

#[test]
fn it_works() {
    assert_eq!(4, adder::add_two(2));
}
```

Here, for example, in our src, we have written a library that provides an `add_two` function, and now we are performing an integration test.

First, use `extern crate` to import this library, since it is the same project, cargo will find it automatically. After importing, just call it according to the usage method of the module, and the other test identifiers are the same as before.

After writing, run `cargo test`, the prompt is similar to the following:

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/lib-c18e7d3494509e74

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

## Document-level tests

Rust's philosophy on documentation is not to write documentation alone. One is that the code itself is the documentation, and the other is that the code comments are the documentation. Rust can not only automatically extract the documents in the code to form a standard form of document collection, but also test the sample code in the documents.

For example, let's add some documentation to the above library:

``````rust
//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```

pub fn add_two(a: i32) -> i32 {
   a + 2
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn it_works() {
      assert_eq!(4, add_two(2));
   }
}
``````


Running `cargo test`, the results are as follows:

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/steve/tmp/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/lib-c18e7d3494509e74

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 2 tests
test add_two_0 ... ok
test _0 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

See it, some more test results.

## Conclusion

We can see that Rust has feature support for testing, documentation, and sample code testing in documentation. From these details, we can see the thoughtfulness and rigor of Rust's design.

However, it is not enough to have good tools. The quality of the project is more determined by the person who writes the code. Under the influence of Rust's rigorous style, we should develop good coding and test writing habits, master certain analysis methods, and implement quality requirements to the end.
