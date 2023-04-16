# conditional branch

-if
- if let
- match

## if expression

The if expression in Rust basically has the following forms:

```rust
// form 1
if expr1 {

}

// form 2
if expr1 {

}
else {

}

// form 3
if expr1 {

}
else if expr2 {
    // else if can be multiple
}
else {

}

```

Compared with C-based languages, the salient features of Rust's if expression are:

1. Judgment conditions are not enclosed in parentheses;
2. It is an expression, not a statement.

In view of the second point above, because it is an expression, we can write the following code:

```rust
let x = 5;

let y = if x == 5 {
    10
} else {
    15
}; // y: i32
```

or compressed into one line:

```rust
let x = 5;

let y = if x == 5 { 10 } else { 15 }; // y: i32
```

## if let

We often see `if let` appearing in pairs in the code, which is actually a simplified usage of match. To illustrate directly with an example:

```rust
let x = Some(5);

if let Some(y) = x {
    println!("{}", y); // here the output is: 5
}

let z = if let Some(y) = x {
     the y
}
else {
    0
};
// z-value is 5

```

The above code is equivalent to

```rust
let x = Some(5);
match x {
    Some(y) => println!("{}", y),
    None => ()
}

let z = match x {
    Some(y) => y,
    None => 0
};
```

The purpose of designing this feature is to directly do a pattern match when the condition is judged, which is convenient for code writing and makes the code more compact.

## match

There is no `switch` keyword like C in Rust, but it has `match` for pattern matching, which can achieve the same function and is much more powerful.

The use of match is very simple, for example:

```rust
let x = 5;

match x {
    1 => {
        println!("one")
    },
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```
Note that match is also an expression. match will be discussed later, see **Pattern Matching** this chapter.
