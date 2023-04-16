# dynamic array Vec
In Chapter 7, we briefly introduced the usage of Vec. In fact, as a very important data type in Rust, mastering the usage of Vec can greatly improve our coding ability in the Rust world.

## Features and declaration methods

Unlike the Array we have been exposed to before, `Vec` has the ability to dynamically add and delete elements, and can perform random access with `O(1)` efficiency. At the same time, the efficiency of the push or pop operation on its tail is equally amortized `O(1)`.
At the same time, there is a very important feature (although most of us do not consider it when programming) that is, all content items of Vec are generated on the heap space, that is to say, you can easily move Vec out A stack without worrying about memory copy affecting execution efficiency - after all, it is just a pointer on the copied stack.

In addition, the generic type `T` in `Vec<T>` must be `Sized`, that is to say, the amount of memory required to store a content item must be known at compile time. For those items whose size is unknown at compile time (function type, etc.), we can use `Box` to wrap it as a pointer.

### new
We can declare a Vec with `std::vec::Vec::new()`.

```rust
let mut v1: Vec<i32> = Vec::new();
```

It should be noted here that the `new` function does not provide a parameter that can explicitly specify its generic type, that is to say, the above code can automatically deduce the generic type of `Vec` according to the type of `v1`; but , you cannot write the following form:

```rust
let mut v1 = Vec::new::<i32>();
// In contrast, the collect function can specify:
// let mut v2 = (0i32..5).collect::<Vec<i32>>();
```

This is because of the declaration form and implementation form of these two functions, and we will not delve into them here.


### Macro declaration

Compared with calling the new function, Rust provides a more intuitive and convenient way to declare a dynamic array: the `vec!` macro.

```rust
let v: Vec<i32> = vec![];

// The following statements are equivalent to:
// let mut temp = Vec::new();
// temp.push(1);
// temp.push(2);
// temp.push(3);
// let v = temp;
let v = vec![1, 2, 3];

let v = vec![0; 10]; //Pay attention to the semicolon, this statement declares a dynamic array of 10 zeros
```

### generate from iterator

Because Vec implements the `FromIterator` trait, we can convert any iterator into a Vec with the help of collect.

```rust
let v: Vec<_> = (1..5).collect();
```

## access and modify

### Random Access

Just like an array, because Vec provides random access capabilities through `Index` and `IndexMut`, we access it through `[index]`. Of course, since there is random access, there will be out-of-bounds problems. In Rust, once the boundary is exceeded, the consequences are extremely serious, which can cause the current thread of Rust to panic. Therefore, unless you are sure what you are doing or in a `for` loop, we do not recommend accessing by subscript.

Here are examples:

```rust
let a = vec![1, 2, 3];
assert_eq!(a[1usize], 2);
```

So, is there a safe subscript access mechanism in Rust? The answer is of course: - `.get(n: usize)` (`.get_mut(n: usize)`) function.
For an array, this function returns an `Option<&T>` (`Option<&mut T>`). When Option==None, the subscript is out of bounds. In other cases, we can safely get the elements in a Vec references.

```rust
let v =vec![1, 2, 3];
assert_eq!(v.get(1), Some(&2));
assert_eq!(v.get(3), None);
```

### Iterators

For a mutable array, Rust provides a simple form of traversal - the for loop.
We can get a reference, mutable reference, ownership of an array.

```rust
let v = vec![1, 2, 3];
for i in &v { .. } // get reference
for i in &mut v { .. } // get mutable reference
for i in v { .. } // Obtain ownership, note that the owner of the Vec will be transferred at this time! !
```

However, it is easy to have multiple layers of `for` loop nesting in this way, so `Vec` provides an `into_iter()` method that can explicitly convert itself into an iterator. But how to use iterators? We will explain in detail in the next chapter.

### push efficiency research

As mentioned earlier, `Vec` has two `O(1)` methods, namely `pop` and `push`, which respectively represent popping or loading data from the end. Theoretically, since `Vec` supports random access, the efficiency of `push` should be consistent. But in fact, because there is memory copying and destruction inside Vec, if you want to fill an array one by one from zero elements until a very huge array is generated at the end, pre-allocate memory for it is a very good way.

Among them, a key method is reserve.

The following code (Note: Since the SystemTime API is only stable after 1.8, please use 1.8.0 stable and above version of rustc to compile):

```rust
use std::time;

fn push_1m(v: &mut Vec<usize>, total: usize) {
    let e = time::SystemTime::now();
    for i in 1..total {
        v.push(i);
    }
    let ed = time::SystemTime::now();
    println!("time spend: {:?}", ed.duration_since(e).unwrap());
}

fn main() {
    let mut v: Vec<usize> = vec![];
    push_1m(&mut v, 5_000_000);
    let mut v: Vec<usize> = vec![];
    v.reserve(5_000_000);
    push_1m(&mut v, 5_000_000);
}
```

On the author's own notebook, the debug version was compiled, and the above code ran out:

```
➜ debug git:(master) ✗ time ./demo
time spend: Duration { secs: 0, nanos: 368875346 }
time spend: Duration { secs: 0, nanos: 259878787 }
./demo 0.62s user 0.01s system 99% cpu 0.632 total

```

Doesn't seem like much of a difference? However, when switching to the release version:

```
➜  release git:(master) ✗ time ./demo
time spend: Duration { secs: 0, nanos: 53389934 }
time spend: Duration { secs: 0, nanos: 24979520 }
./demo  0.06s user 0.02s system 97% cpu 0.082 total
```

Note the number of digits spent in time. It can be seen that after removing the debugging information of the debug version, whether the pre-allocation memory consumption time is doubled!

Such results show that pre-allocating memory does help improve efficiency.

Some people may ask, if you are so entangled with this time, isn't it also saved at the nanosecond level in the end, is it meaningful? Of course it makes sense.

First, nanoseconds are also time, again because the `Vec` for this test is just the simplest memory structure. Once the copying of large objects is involved, the time spent may not necessarily be so small.
Second, frequent application and deletion of heap space, once the memory reaches the bottleneck, your program will be extremely dangerous.

For more operations on `Vec`, please refer to the api of the standard library.
