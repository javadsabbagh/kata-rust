# Closure as parameter and return value
## Closures as arguments (Taking closures as arguments)

Now that we know that closures are traits, we know how to accept and return closures; just like any other trait!

This also means that we can also choose static or dynamic distribution. First, let's write a function that takes a callable structure, calls it, and returns the result:

```rust
fn call_with_one<F>(some_closure: F) -> i32
    where F : Fn(i32) -> i32 {

    some_closure(1)
}

let answer = call_with_one(|x| x + 2);

assert_eq!(3, answer);
```

We pass our closure, `|x| x + 2`, to `call_with_one`. It does exactly what we say: it calls the closure with `1` as an argument.

Let's dig deeper into the signature of `call_with_one`:

```rust
fn call_with_one<F>(some_closure: F) -> i32
#    where F : Fn(i32) -> i32 {
#    some_closure(1) }
```

We get a parameter, and it has type `F`. We also return an `i32`. This part is not fun. The next part is:

```rust
# fn call_with_one<F>(some_closure: F) -> i32
    where F : Fn(i32) -> i32 {
#   some_closure(1) }
```

Since `Fn` is a trait, we can use it to limit our generics. In this example, our closure takes an `i32` as a parameter and returns an `i32`, so we use generics to restrict `Fn(i32) -> i32`.

There is another key point here: since we constrain the generic with a trait, it will be monomorphic, and therefore, we use static dispatch in the closure. It's very simple. In many languages, closures are permanently allocated on the heap, so they are always dispatched dynamically. In Rust, we can allocate our closure's environment on the stack and dispatch calls statically. This often happens with iterators and their adapters, which often take closures as parameters.

Of course, if we want dynamic distribution, we can do that too. The trait object handles this situation, usually:

```rust
fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

let answer = call_with_one(&|x| x + 2);

assert_eq!(3, answer);
```

Now we get a trait object, a `&Fn`. And when we pass our closure to `call_with_one` we have to get a reference, so we use `&||`.

## Function pointers and closures

A function pointer is a bit like a closure without an environment. Therefore, you can pass a function pointer to any function except as a closure parameter, the following code will work:

```rust
fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

fn add_one(i: i32) -> i32 {
    i + 1
}

let f = add_one;

let answer = call_with_one(&f);

assert_eq!(2, answer);
```

In this example, we don't strictly need this intermediate variable `f`, the name of the function is fine:

```rust
let answer = call_with_one(&add_one);
```

## Returning closures

Returning closures in various situations is very common for functional style code. If you try to return a closure, you may get an error. At first glance, this may seem a little strange, but we'll figure it out. When you try to return a closure from a function, you might write code like this:

```rust
fn factory() -> (Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

Compiling gives this long list of related errors:

```text
error: the trait `core::marker::Sized` is not implemented for the type
`core::ops::Fn(i32) -> i32` [E0277]
fn factory() -> (Fn(i32) -> i32) {
                ^~~~~~~~~~~~~~~~
note: `core::ops::Fn(i32) -> i32` does not have a constant size known at compile-time
fn factory() -> (Fn(i32) -> i32) {
                ^~~~~~~~~~~~~~~~
error: the trait `core::marker::Sized` is not implemented for the type `core::ops::Fn(i32) -> i32` [E0277]
let f = factory();
    ^
note: `core::ops::Fn(i32) -> i32` does not have a constant size known at compile-time
let f = factory();
    ^
```

In order to return something from a function, Rust needs to know the size of the return type. But `Fn` is a trait, and it can be anything of various sizes. For example, the return value can be any type that implements `Fn`. A simple solution is: return a reference. Because the size of the reference is fixed, the size of the return value is fixed. Therefore we can write:

```rust
fn factory() -> &(Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

But this will cause another error:

```text
error: missing lifetime specifier [E0106]
fn factory() -> &(Fn(i32) -> i32) {
                ^~~~~~~~~~~~~~~~~
```

right. Since we have a reference, we need to give it a lifetime. However our `factory()` function does not take arguments, so elision cannot be used here. What life cycle can we use? ``static`:

```rust
fn factory() -> &'static (Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

But then another error occurs:

```text
error: mismatched types:
 expected `&'static core::ops::Fn(i32) -> i32`,
    found `[closure@<anon>:7:9: 7:20]`
(expected &-ptr,
    found closure) [E0308]
         |x| x + num
         ^~~~~~~~~~~

```

This error lets us know that instead of returning a `&'static Fn(i32) -> i32`, we returned a `[closure <anon>:7:9:7:20]`. wait, what?

Because each closure generates its own environment `struct` and implements `Fn` and other things, these types are anonymous. They only exist within this closure. So Rust shows them as `closure <anon>` instead of some auto-generated name.

This error also indicates that the return type is expected to be a reference, but what we are trying to return is not. Furthermore, we cannot directly give an object a `'static` statement cycle. So we change the method and wrap `Fn` by `Box` to return a trait object. This *almost* works successfully:

```rust
fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(|x| x + num)
}
# fn main() {
let f = factory();

let answer = f(1);
assert_eq!(6, answer);
# }
```

Here's one last question:

```text
error: closure may outlive the current function, but it borrows `num`,
which is owned by the current function [E0373]
Box::new(|x| x + num)
         ^~~~~~~~~~~
```

Well, as we discussed above, closures borrow their environment. And in this example. Our environment is based on a stack-allocated `5`, `num` variable binding. So this borrow has the lifetime of this stack frame. So if we return this closure, the function call will end and the stack frame will disappear, so our closure points to the freed memory environment! With one last modification, we can make it work:

```rust
fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}
# fn main() {
let f = factory();

let answer = f(1);
assert_eq!(6, answer);
# }
```

By adding the `move` keyword to the inner closure, we force the closure to use move to capture environment variables. Because the num type here is i32, in fact, the move here executes copy, so that the closure no longer has a pointer to the environment, but completely owns the captured variable. and allow it to leave our stack frame.

> ### This part is quoted from [The Rust Programming Language Chinese version](https://github.com/KaiserY/rust-book-chinese/blob/master/content/Closures%20%E9%97%AD%E5 %8C%85.md)
