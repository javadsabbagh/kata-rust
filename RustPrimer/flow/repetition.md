# loop

- for
-while
-loop
- break and continue
- label


## for

The for statement is used to iterate over an iterator.

```rust
for var in iterator {
    code
}
```

Rust iterators return a sequence of elements, each element being an iteration of the loop. Its value is then bound to var, which is valid in the body of the loop. Whenever the body of the loop finishes executing, we fetch the next value from the iterator, and then we repeat. The for loop ends when there are no more values in the iterator.

for example:

```rust
for x in 0..10 {
    println!("{}", x); // x: i32
}
```

output

```
0
1
2
3
4
5
6
7
8
9
```

Students who are not familiar with the concept of iterators may be dumbfounded. Let’s use the for statement in C form for comparison:

```rust
// Example of for loop in C language
for (x = 0; x < 10; x++) {
    printf( "%d\n", x );
}
```

The output is the same, so why did Rust design the for statement this way?

1. Simplify the determination of boundary conditions and reduce errors;
2. Reduce runtime bounds checks and improve performance.

Even for an experienced C developer, manually controlling each element to loop over is complex and error-prone.

The for statement is syntactic sugar for iterator traversal.

Although the form of the above iterator is good, it seems that the index information is missing during the loop. Rust takes this into account, and when you need to keep track of how many times you've looped, you can use the `.enumerate()` function. for example:

```rust
for (i,j) in (5..10).enumerate() {
    println!("i = {} and j = {}", i, j);
}
```

output:

```
i = 0 and j = 5
i = 1 and j = 6
i = 2 and j = 7
i = 3 and j = 8
i = 4 and j = 9
```

Another example:

```rust
let lines = "Content of line one
Content of line two
Content of line three
Content of line four".lines();
for (linenumber, line) in lines.enumerate() {
    println!("{}: {}", linenumber, line);
}
```

output:

```
0: Content of line one
1: Content of line two
2: Content of line three
3: Content of line four
```

For more information about iterators, see **Iterators** chapter.

## while

Rust provides a while statement, which executes the body of the statement while a conditional expression is true. Choose while when you are not sure how many times you should loop.

```rust
while expression {
    code
}
```

for example:

```rust
let mut x = 5; // mut x: i32
let mut done = false; // mut done: bool

while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}
```

##loop

There is a situation, we often encounter, is to write an infinite loop:

```rust
while true {
    // do something
}
```

For this situation, Rust provides a statement loop specially optimized.

```rust
loop {
    // do something
}
```

The main difference between `loop` and `while true` lies in the static analysis during compilation.

For example, the following code:

```rust
let mut a;
loop {
     a = 1;
     // ... break ...
}
do_something(a)
```

If it is a `loop` cycle, the compiler will correctly analyze that the variable `a` will be correctly initialized, but if it is replaced by `while true`, a compilation error will occur. This small difference also affects life cycle analysis.

## break and continue

Similar to the C language, Rust also provides two keywords, break and continue, to control the flow of the loop.

- break is used to break out of the loop of the current layer;
- continue is used to execute the next iteration of the current layer.

Like the while example above:

```rust
let mut x = 5;
let mut done = false;

while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}
```

Can be optimized to:

```rust
let mut x = 5;

loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
}
```

This feels a little more intuitive.

The following example demonstrates the use of continue:

```rust
for x in 0..10 {
    if x % 2 == 0 { continue; }

    println!("{}", x);
}
```

Its function is to print out the odd numbers `0~9`. The result is as follows:

```
1
3
5
7
9
```

##label

You may run into situations where you have nested loops and wish to specify which of your break or continue should work. Like most languages, the default break or continue will act on the loop at the current level. When you want a break or continue to be applied to an outer loop, you can use a label to specify which loop your break or continue statement applies to.

The following code will only print x and y if they are both odd:

```rust
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        println!("x: {}, y: {}", x, y);
    }
}
```
