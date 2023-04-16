## Parallel
Theoretically, parallelism has nothing to do with language, so you can try to use Rust to achieve parallelism in theory. This section will not comprehensively introduce specific parallel theoretical knowledge in detail, but only introduce how to use Rust to implement related parallel modes.

A major feature of Rust is that "thread safety" can be guaranteed. Also, there is no performance loss. What's more interesting is that the Rust compiler actually only has basic abstractions such as `Send` and `Sync`, but knows nothing about basic parallel-related concepts such as "threads", "locks" and "synchronization". Achieved. This means that Rust's implementation of parallel programming can have relatively good scalability, and it is easy to use libraries to support those common parallel programming patterns.
Below, we use an example to demonstrate how Rust combines thread safety/execution efficiency/simple use.

In graphics programming, we often have to deal with the problem of normalization: that is, converting a value within a range to a value within a range of 1. For example, a color value of 255 is normalized to 1. Assuming we have an array representing color values to be normalized, it is very simple to deal with it in a non-parallelized way, you can try it yourself. Next, we will use parallelization to process, and divide the values in the array at the same time to multiple threads for parallel normalization processing.

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut colors = [-20.0f32, 0.0, 20.0, 40.0,
        80.0, 100.0, 150.0, 180.0, 200.0, 250.0, 300.0];
    println!("original:    {:?}", &colors);

    colors.par_iter_mut().for_each(|color| {
        let c : f32 = if *color < 0.0 {
                0.0
            } else if *color > 255.0 {
                255.0
            } else {
                *color
            };
        *color = c / 255.0;
    });
    println!("transformed: {:?}", &colors);
}
```

operation result:

```
original:    [-20, 0, 20, 40, 80, 100, 150, 180, 200, 250, 300]
transformed: [0, 0, 0.078431375, 0.15686275, 0.3137255, 0.39215687, 0.5882353, 0.7058824, 0.78431374, 0.98039216, 1]
```

Isn't the above code very simple. Calling `par_iter_mut` obtains a writeable iterator that executes in parallel, and `for_each` performs an operation on each element. That's all.
We can do this task so easily because we introduced the [rayon](https://github.com/nikomatsakis/rayon/) library. It does all the dirty work and exposes a clear, safe and easy-to-use interface to us. Rust can also implement more advanced parallel program development modes such as asynchronous IO and coroutines completely in the form of libraries.

In order to deepen the understanding and practice of Rust concurrent programming, a challenge task is also arranged: to implement a Rust version of MapReduce mode. It is worth your challenge.
