# control flow

## If

If is a special form of branch, `else` and `else if` can also be used.
Unlike the C language, logical conditions do not need to be enclosed in parentheses, but the condition must be followed by a code block.
An `if` in Rust is an expression that can be assigned to a variable:

```rust
let x = 5;

let y = if x == 5 { 10 } else { 15 };
```

Rust is an expression-based programming language with one and only two statements:

1. **Declaration statement** (declaration statement), such as the `let` statement for variable binding.
2. **Expression statement** (expression statement), it turns the expression into a statement by adding a semicolon `;` at the end,
The value of this expression is discarded and unit`()` is always returned.

If the expression returns, it always returns a value, but the statement does not return a value or returns `()`, so the following code will report an error:

```rust
let y = (let x = 5);

let z: i32 = if x == 5 { 10; } else { 15; };
```

It's worth noting that in Rust an assignment (such as `x = 5`) is also an expression, returning the value of the unit `()`.

## For

The `for` loop in Rust is very different from the C language style, and the abstract structure is as follows:

```rust
for var in expression {
     code
}
```

Where `expression` is an iterator (iterator), the specific example is `0..10` (excluding the last value),
Or `[0, 1, 2].iter()`.

## While

The `while` loop in Rust is similar to the one in C. For infinite loops, Rust has a dedicated keyword `loop`.
If you need to exit the loop early, you can use the keyword `break` or `continue`,
It is also allowed to set a label at the beginning of the loop (also applies to `for` loops):

```rust
'outer: loop {
    println!("Entered the outer loop");

    'inner: loop {
        println!("Entered the inner loop");
        break 'outer;
    }

    println!("This point will never be reached");
}

println!("Exited the outer loop");
```

## Match

The `match` expression in Rust is very powerful, first look at an example:

```rust
let day = 5;

match day {
   0 | 6 => println!("weekend"),
   1 ... 5 => println!("weekday"),
   _ => println!("invalid"),
}
```

Where `|` is used to match multiple values, `...` matches a range (including the last value), and `_` is required here,
Because `match` enforces exhaustiveness checking, all possible values must be covered.
If you need to get the value matched by `|` or `...`, you can use `@` bind variable:

```rust
let x = 1;

match x {
     e @ 1 ... 5 => println!("got a range element {}", e),
     _ => println!("anything"),
}
```

Use the `ref` keyword to get a reference:

```rust
let x = 5;
let mut y = 5;

match x {
     // the `r` inside the match has the type `&i32`
     ref r => println!("Got a reference to {}", r),
}

match y {
     // the `mr` inside the match has the type `&i32` and is mutable
     ref mut mr => println!("Got a mutable reference to {}", mr),
}
```

Here's another example of using a `match` expression to destructure a tuple:

```rust
let pair = (0, -2);

match pair {
     (0, y) => println!("x is `0` and `y` is `{:?}`", y),
     (x, 0) => println!("`x` is `{:?}` and y is `0`", x),
     _ => println!("It doesn't matter what they are"),
}
```

This deconstruction of `match` also applies to structs or enums. If necessary, you can also use `..` to ignore fields or data:

```rust
struct Point {
     x: i32,
     y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
     Point { x, .. } => println!("x is {}", x),
}

enum OptionalInt {
     Value(i32),
     Missing,
}

let x = OptionalInt::Value(5);

match x {
     // Here is the if guard expression of match, we will introduce it in detail in later chapters
     OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
     OptionalInt::Value(..) => println!("Got an int!"),
     OptionalInt::Missing => println!("No such luck."),
}
```

Additionally, Rust introduces `if let` and `while let` for pattern matching:

```rust
let number = Some(7);
let mut optional = Some(0);

// If `let` destroys `number` into `Some(i)`, evaluate the block.
if let Some(i) = number {
    println!("Matched {:?}!", i);
} else {
    println!("Didn't match a number!");
}

// While `let` destroys `optional` into `Some(i)`, evaluate the block.
while let Some(i) = optional {
    if i > 9 {
        println!("Greater than 9, quit!");
        optional = None;
    } else {
        println!("`i` is `{:?}`. Try again.", i);
        optional = Some(i + 1);
    }
}
```
