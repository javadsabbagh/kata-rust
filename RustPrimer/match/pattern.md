# model
Patterns are another powerful feature of Rust. It can be used in `let` and `match` expressions. I believe you should still remember the example we mentioned in [Compound Types](../type/compound-types.md) about deconstructing tuples in let expressions. In fact, this is a pattern.

```rust
let tup = (0u8, 1u8);
let (x, y) = tup;
```

And what we need to know is that if a binding with the same name that already exists in the current scope appears in a mode, it will overwrite the external binding. for example:

```rust
let x = 1;
let c = 'c';

match c {
    x => println!("x: {} c: {}", x, c),
}

println!("x: {}", x);
```

Its output is:

```
x: c c: c
x: 1
```

In the above code, the binding of `x` in the match scope is overwritten to `'c'`, and out of this scope, the binding of `x` is restored to `1`. This is consistent with the behavior of variable binding.

## More powerful deconstruction

In the previous section, we got a preliminary understanding of the convenience of pattern matching when deconstructing `enum`. In fact, in Rust, patterns can be used to deconstruct any composite type - struct/tuple/enum. Now we're going to talk about a more complicated example, destructuring `struct`.

First, we can perform standard destructuring on a struct:

```rust
struct Point {
    x: i64,
    y: i64,
}
let point = Point { x: 0, y: 0 };
match point {
    Point { x, y } => println!("({},{})", x, y),
}
```

Finally, we get the value inside `Point`. Someone said, what should I do if I want to change my name?
Very simple, you can use `:` to rename the fields of a struct, as follows:

```rust
struct Point {
    x: i64,
    y: i64,
}
let point = Point { x: 0, y: 0 };
match point {
    Point { x: x1, y: y1} => println!("({},{})", x1, y1),
}
```

In addition, sometimes we are only interested in certain fields, so we can use `..` to omit other fields.

```rust
struct Point {
    x: i64,
    y: i64,
}

let point = Point { x: 0, y: 0 };

match point {
    Point { y, .. } => println!("y is {}", y),
}
```

## Ignoring and memory management

To summarize, we encountered two different cases of pattern ignoring - `_` and `..`. It should be noted here that fields that are ignored in pattern matching will not be `move`, and those that implement `Copy` will be copied first instead of `move`.

It’s a bit of a mouthful, here’s the code:

```rust
let tuple: (u32, String) = (5, String::from("five"));

let (x, s) = tuple;

// The following line will cause a compilation error, because the String type does not implement Copy, so the tuple is moved as a whole.
// println!("Tuple is: {:?}", tuple);

let tuple = (5, String::from("five"));

// Ignore the String type, and u32 implements Copy, then the tuple will not be moved
let (x, _) = tuple;

println!("Tuple is: {:?}", tuple);
```

## ranges and multiple matches

Pattern matching can be used to match a single possibility, and of course it can also be used to match multiple situations:

### scope

In pattern matching, when I want to match a range of numbers (characters), we can use `...` to express:

```rust
let x = 1;

match x {
     1 ... 10 => println!("one to ten"),
     _ => println!("other"),
}

let c = 'w';

match c {
     'a' ... 'z' => println!("lowercase letter"),
     'A' ... 'Z' => println!("Capital letter"),
     _ => println!("Other characters"),
}
```

### Multiple matches

When we simply want to match multiple conditions, we can use `|` to separate multiple matching conditions

```rust
let x = 1;

match x {
     1 | 2 => println!("one or two"),
     _ => println!("other"),
}
```

## ref and ref mut

We learned earlier that when hit by pattern matching, the type that does not implement `Copy` will be moved by default, so the original owner no longer holds its ownership. But sometimes, we just want to get a (mutable) reference to a variable from it, but don't want to move it out of scope. How to do it? Answer: Use `ref` or `ref mut`.

```rust
let mut x = 5;

match x {
    ref mut mr => println!("mut ref :{}", mr),
}
// Of course... it can also be used in let expressions
let ref mut mrx = x;
```


## Variable Binding

In the process of pattern matching, we can use `@` to bind a variable name, which is very convenient in complex pattern matching. For example, a named range matches as follows:

```rust
let x = 1u32;
match x {
    e @ 1 ... 5 | e @ 10 ... 15 => println!("get:{}", e),
    _ => (),
}
```

As the code shows, e is bound to the value of x.

Of course, variable binding is an extremely useful syntax, here is an example from the official doc:

```rust
#[derive(Debug)]
struct Person {
    name: Option<String>,
}

let name = "Steve".to_string();
let x: Option<Person> = Some(Person { name: Some(name) });
match x {
    Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
    _ => {}
}
```

output:

```
Some("Steve")
```

## Postconditions

A post-if expression can be placed after the match pattern, called `match guards`. For example the following code:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
}
```

Guess the output of the code above?

The answer is `no`. Because guard is a post-condition, it is the post-condition of the entire match: so the logic expressed by the above formula is actually:

```
// Pseudocode representation
IF y AND (x IN List[4, 5])
```
