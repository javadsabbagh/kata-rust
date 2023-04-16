# Closure syntax
## Basic form
The closure looks like this:

```rust
let plus_one = |x: i32| x + 1;

assert_eq!(2, plus_one(1));
```

We create a binding, `plus_one`, and assign it a closure. The parameters of the closure are inside the pipe (`|`), and the body of the closure is an expression, in this case, `x + 1`. Remember that `{}` is an expression, so we can also have closures with multiple lines:

```rust
let plus_two = |x| {
    let mut result: i32 = x;

    result += 1;
    result += 1;

    result
};

assert_eq!(4, plus_two(2));
```

You'll notice that some aspects of closures are a bit different than regular functions defined with `fn`. The first is that we don't need to specify the types of the closure's receiving and returning parameters. We can:

```rust
let plus_one = |x: i32| -> i32 { x + 1 };

assert_eq!(2, plus_one(1));
```

But we don't need to write that. why? Basically, it's for "ergonomic" reasons. Because specifying full types for named functions helps with things like documentation and type inference, whereas types for closures are less documented because they are anonymous and don't cause "distant errors" like inferring the type of a named function ".

The syntax for the second is much the same. I'll add spaces to make them look a bit more like:

```rust
fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
let plus_one_v2 = |x: i32| -> i32 { x + 1 };
let plus_one_v3 = |x: i32|          x + 1  ;
```

## capture variables
They are called "closures" because they are "close over their environment". This looks like:

```rust
let num = 5;
let plus_num = |x: i32| x + num;

assert_eq!(10, plus_num(5));
```

This closure, `plus_num`, references the `let` binding in its scope: `num`. More specifically, it borrows bindings. If we do something that would conflict with this binding, we'll get an error. For example this:

```rust
let mut num = 5;
let plus_num = |x: i32| x + num;

let y = &mut num;
```

The error is:

```text
error: cannot borrow `num` as mutable because it is also borrowed as immutable
    let y = &mut num;
                 ^~~
note: previous borrow of `num` occurs here due to use in closure; the immutable
  borrow prevents subsequent moves or mutable borrows of `num` until the borrow
  ends
    let plus_num = |x| x + num;
                   ^~~~~~~~~~~
note: previous borrow ends here
fn main() {
    let mut num = 5;
    let plus_num = |x| x + num;

    let y = &mut num;
}
^
```

A long-winded but useful error message! As it says, we can't take a mutable borrow of `num` because the closure already borrows it. If we let the closure go out of scope, we can:

```rust
let mut num = 5;
{
    let plus_num = |x: i32| x + num;

} // plus_num goes out of scope, borrow of num ends

let y = &mut num;
```

If your closure needs it, Rust will take ownership and move the environment:

```rust
let nums = vec![1, 2, 3];

let takes_nums = || nums;

println!("{:?}", nums);
```

This will give us:

```text
note: `nums` moved into closure environment here because it has type
  `[closure(()) -> collections::vec::Vec<i32>]`, which is non-copyable
let takes_nums = || nums;
                    ^~~~~~~
```

`Vec<T>` takes ownership of its contents, and for this reason we must take ownership of `nums` when we refer to it in closures. This is the same as if we passed `nums` to a function that took ownership of it.

## move closure
We can force our closure to take ownership of its environment using the `move` keyword:

```rust
let num = 5;

let owns_num = move |x: i32| x + num;
```

Now, even if the keyword is `move`, variables follow normal move semantics. In this example, `5` implements `Copy`, so `owns_num` takes ownership of a copy of `5`. So what's the difference?

```rust
let mut num = 5;

{
    let mut add_num = |x: i32| num += x;

    add_num(5);
}

assert_eq!(10, num);
```

So in this example, our closure takes a mutable reference to `num`, and then we call `add_num`, which changes the value in it, as we expect. We also need to declare `add_num` as `mut`, because we will change its environment.

If we modify the closure with `move`, something will happen:

```rust
let mut num = 5;

{
    let mut add_num = move |x: i32| num += x;

    add_num(5);
}

assert_eq!(5, num);
```

We will only get `5`. This time we didn't get the mutable borrow of the external `num`, we actually moved `num` into the closure. Because `num` has the Copy attribute, after the move occurs, the life cycle of the previous variable is not over, and it can continue to be used in `assert_eq!`. The variable we print and the variable inside the closure are two independent variables. If the environment variable we capture is not Copy, then after the external environment variable is moved into the closure,
It can no longer be used in the original function, it can only be used in the closure.

But before we talk about getting or returning a closure, we should look a little more at how closures are implemented. As a systems language, Rust gives you a lot of control over your code, and closures do the same.

> ### This part is quoted from [The Rust Programming Language Chinese version](https://github.com/KaiserY/rust-book-chinese/blob/master/content/Closures%20%E9%97%AD%E5 %8C%85.md)
