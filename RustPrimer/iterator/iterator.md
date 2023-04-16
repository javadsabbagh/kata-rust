# iterator

## Starting from the for loop

We have learned Rust's `for` loop expression in the control statement. We know that Rust's for loop is actually different from the loop statement of C language. Why is this? Because, the `for` loop is nothing but syntactic sugar provided by the Rust compiler!

First of all, we know that Rust has a `for` loop that can sequentially access any element of the iterator, namely:

```rust
for i in 1..10 {
    println!("{}", i);
}
```

Here we know that (1..10) itself is an iterator, and we can call the `.next()` method on this iterator, so the `for` loop can completely traverse a loop.
And for `Vec`:

```
let values = vec![1,2,3];
for x in values {
    println!("{}", x);
}
```

In the code above, we didn't explicitly convert a `Vec` to an iterator, so how does it work? Students who open the standard library and flip the api now may find that `Vec` itself does not implement `Iterator`, that is to say, you cannot call the `.next()` method on `Vec` itself. However, when we searched, we found that `Vec` implements the `IntoIterator` trait.

In fact, what the `for` loop really loops is not an iterator (Iterator), what really works in this syntactic sugar is the `IntoIterator` trait.

Therefore, the above code can be expanded into the following equivalent code (just for illustration, no guarantee of successful compilation):

```rust
let values = vec![1, 2, 3];

{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => { println!("{}", x); },
                None => break,
            }
        },
    };
    result
}
```

In this code, we first call `into_iter` on `Vec` to determine whether it can be converted into an iterator, and if so, iterate.

So, what about the iterator itself?

For this, Rust provides an implementation in the standard library:

```rust
impl<I: Iterator> IntoIterator for I {
    // ...
}
```

In other words, Rust implements `IntoIterator` for all iterators by default. This implementation is very simple, just return itself every time.

That is to say:

Any `Iterator` can be used in a `for` loop!

### infinite iterator

Rust supports generating an infinite-length auto-increment sequence by omitting the high bits, namely:

```rust
let inf_seq = (1..).into_iter();
```

But don't worry about this infinitely growing sequence bursting your memory and occupying your CPU, because the adapter is inert and itself is safe unless you `collect` or `fold` this sequence!
However, I think you are as smart as you, and you won't make such a mistake!
So, to apply this, you need to truncate it with `take` or `take_while`, must? Unless you use it as a generator. Of course, that's another story.

## Consumer and Adapter

After talking about the `for` loop, we roughly figured out the relationship between `Iterator` and `IntoIterator`. Let's talk about consumers and adapters.

A consumer is a special operation on an iterator whose main purpose is to convert an iterator into a value of another type, not another iterator.

The adapter traverses the iterator, and the result generated is another iterator, which can be directly called by the chain call.

From the above inference, we can draw: *Iterator is actually a kind of adapter! *

### Consumers

Just like the producer-consumer model that everyone is familiar with, the iterator is responsible for production, and the consumer is responsible for the final transformation of the produced things. A typical consumer is `collect`. Earlier we wrote about `collect` related operations, which are responsible for taking out all the data in the iterator, such as the following operations:

```rust
let v = (1..20).collect(); // Compilation fails!
```

Try running the above code, only to find that the compiler won't let you through. Because you didn't specify the type! What type to specify? It turns out that collect only knows to collect iterators into a type that implements `FromIterator`. However, in fact, there are many types that implement this trait (Vec, HashMap, etc.), so collect does not have a context to judge that v should be based on What a way to collect! !

To solve this problem, we have two solutions:

1. Explicitly mark the type of `v`:

    ```rust
    let v: Vec<_> = (1..20).collect();
    ```

2. Explicitly specify the type of `collect` call:

    ```rust
    let v = (1..20).collect::<Vec<_>>();
    ```

Of course, there are other consumers in an iterator, such as the `.nth()` function used to get the number of values, and the `.find()` function used to find the value, calling `next for the next value () `Functions and so on, we cannot introduce them one by one here due to space limitations. So, below we only introduce another commonly used consumer - `fold`.

Of course, you may not feel much about the name in Rust. In fact, the `fold` function is just the Reduce function in the famous MapReduce (the slight difference is that this Reduce has an initial value).

The `fold` function has the following form:

```rust
fold(base, |accumulator, element| .. )
```

We can write this as an example:

```rust
let m = (1..20).fold(1u64, |mul, x| mul*x);
```

It should be noted that the type of the output result of `fold` is finally consistent with the type of `base` (if the type of `base` is not specified, then it can be reversed according to the type of `m` in front, unless` The type of m` is also unspecified), that is to say, once we change the `base` in the above code from `1u64` to `1`, then this line of code will eventually crash due to data overflow!

### Adapter

In the production and consumption model that we are familiar with, the things produced by producers may not be bought by consumers. Therefore, the original products need to be reassembled. This reassembly process is the adapter. Because the adapter returns a new iterator, it can be written directly with chain requests.

The Reduce function was mentioned earlier, so naturally I have to mention another supporting function - `map`:

Students who are familiar with the Python language must know that there is a built-in `map` function in Python, which can transform the value of one iterator into another. The `map` function in Rust actually does the same thing, and even the calling method is surprisingly similar!

```rust
(1..20).map(|x| x+1);
```

The above code shows a "increment all elements of the iterator" operation, however, if you try to compile this code, the compiler will give you a hint:

```
warning: unused result which must be used: iterator adaptors are lazy and
         do nothing unless consumed, #[warn(unused_must_use)] on by default
(1..20).map(|x| x + 1);
 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

Yeah, what is this?

Because all adapters are lazy evaluated!

**That is to say, unless you call a consumer, your operation will never be called! **

Now, we know `map`, so people who are familiar with Python say, is there still `filter`! ? Answer, yes... the usage is similar. `filter` accepts a closure function and returns a boolean value. When `true` is returned, it means to keep it, and `false` is discarded.

```rust
let v: Vec<_> = (1..20).filter(|x| x%2 == 0).collect();
```

The above code means to filter out all even numbers.

## other

Above we learned the basic concepts of iterators, adapters, and consumers. The following examples will introduce other adapters and consumers in Rust.

### skip and take

The function of `take(n)` is to take the first `n` elements, while `skip(n)` is just the opposite, skipping the first `n` elements.

```rust
let v = vec![1, 2, 3, 4, 5, 6];
let v_take = v.iter()
    .cloned()
    .take(2)
    .collect::<Vec<_>>();
assert_eq!(v_take, vec![1, 2]);

let v_skip: Vec<_> = v.iter()
    .cloned()
    .skip(2)
    .collect();
assert_eq!(v_skip, vec![3, 4, 5, 6]);
```

### The love and hatred of zip and enumerate

`zip` is an adapter, its function is to compress the contents of two iterators together to form a new iterator like `Iterator<Item=(ValueFromA, ValueFromB)>`;

```rust
let names = vec!["WaySLOG", "Mike", "Elton"];
let scores = vec![60, 80, 100];
let score_map: HashMap<_, _> = names.iter()
    .zip(scores.iter())
    .collect();
println!("{:?}", score_map);
```

And `enumerate`, the familiar Python students called again: Python also has it! Yes, the effect is the same, that is, to display the subscript of the iterator, namely:

```rust
let v = vec![1u64, 2, 3, 4, 5, 6];
let val = v.iter()
     .enumerate()
     // Iterate to generate the mark, and remove one every two elements
     .filter(|&(idx, _)| idx % 2 == 0)
     // Remove the subscript, if you call unzip to get the final result, you can call the following sentence to terminate the chain call
     // .unzip::<_,_, vec<_>, vec<_>>().1
     .map(|(idx, val)| val)
     // add up 1+3+5 = 9
     .fold(0u64, |sum, acm| sum + acm);

println!("{}", val);
```

### A series of lookup functions

Rust's iterators have a series of lookup functions, such as:

* `find()`: Pass in a closure function, search for the first element that can make this closure return `true` from the beginning to the end, and return `Option<Item>`
* `position()`: Similar to the `find` function, but this time the output is `Option<usize>`, the number of elements.
* `all()`: Pass in a function, if calling this function returns `false` for any element, then the entire expression returns `false`, otherwise it returns `true`
* `any()`: Similar to `all()`, but this time if any one returns `true`, the entire expression returns `true`, otherwise `false`
* `max()` and `min()`: Find all elements in the entire iterator and return the element with the largest or smallest value. Note: Because of the `PartialOrder` mentioned in Chapter 7, `max` and `min` will have unexpected results when they are applied to floating-point numbers.


The above are some commonly used iterators and adapters and their usage, just for popular science, for this chapter. I hope that everyone can practice more to understand, rather than rote memorization.

Well, leave an exercise:

## Exercises

Use an iterator to generate an ascending sequence of daffodil numbers with a length of 10, then reverse the sequence, and output
