# Performance Testing

Unit testing is used to verify the correctness of the program. However, after the program can run normally, it is often necessary to test the execution speed of the program (part). At this time, f needs to use performance testing.
Generally speaking, the so-called performance test refers to measuring the running speed of the program, that is, how long it takes to run once (usually performing multiple averages). Rust even integrates this feature into the basic language features. It is really a language that attaches great importance to engineering.

The following directly explains how to use it.

```
cargo new bench
cd bench
```

Edit the `src/lib.rs` file and add the following code in it:

```rust
#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
}
```

Notice:

1. Although `extern crate test;` is used here, there is no need to add a dependency on `test` in the dependency area of the `Cargo.toml` file of the project;
2. The evaluation function `fn bench_add_two(b: &mut Bencher) {}` is marked with `#[bench]`, and the function accepts a parameter, `b` is the benchmark provided by Rust. This wording is fixed.

Then, in the project root directory, execute

```
cargo bench
```

The output is similar to the following:

```
$ cargo bench
   Compiling benchit v0.0.1 (file:///home/mike/tmp/benchit)
     Running target/release/benchit-91b3e234d4ed382a

running 2 tests
test tests::it_works ... ignored
test tests::bench_add_two ... bench:         1 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured
```

As you can see, Rust's performance tests are in nanoseconds ns.

When writing evaluation code, you need to pay attention to the following points:

1. Only put the code (function) you need to do performance testing in the evaluation function;
2. For codes (functions) involved in performance testing, each test is required to do the same thing, and do not perform operations that accumulate and change external states;
3. The code (function) of the parameter performance test should not take too long to execute. If it is too long, it is best to test it in several parts. This also makes it easy to find out where performance bottlenecks are.
