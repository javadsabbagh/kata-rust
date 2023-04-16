# Closure implementation

Rust's implementation of closures is a little different than other languages. They are actually syntactic sugar for traits. Before doing this you'll want to read the [traits chapter](https://doc.rust-lang.org/stable/book/traits.html), and the [trait objects](https://doc.rust-lang.org /stable/book/trait-objects.html).

Do you understand? very good.

The key to understanding how closures work under the hood is a bit odd: using `()` to call a function, like `foo()`, is an overloadable operator. At this point, everything else will become clear. In Rust, we use the trait system to overload operators. Calling functions is no exception. We have three traits to overload separately:

```rust
# mod foo {
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
# }
```

You'll notice some differences between these traits, but one big difference is `self`: `Fn` gets `&self`, `FnMut` gets `&mut self`, and `FnOnce` gets `self`. This includes all 3 kinds of `self` via the usual function call syntax. But we group them into 3 traits instead of 1 alone. This gives us a lot of control over what kind of closures we can use.

The `|| {}` syntax of the closure is the syntactic sugar of the above 3 traits. Rust will create a struct for the environment, `impl` the appropriate trait, and use it.

> ### This part is quoted from [The Rust Programming Language Chinese version](https://github.com/KaiserY/rust-book-chinese/blob/master/content/Closures%20%E9%97%AD%E5 %8C%85.md)
