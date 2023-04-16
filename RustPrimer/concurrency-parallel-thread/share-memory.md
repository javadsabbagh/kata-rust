## Shared memory
In addition to message passing, there is a well-known concurrency model known as shared memory. In fact, if memory cannot be shared, message passing cannot pass messages between different threads, nor can it talk about waiting and notification between different threads. Shared memory is the basis for all this to happen. If you look at the source code, you will find that the internal implementation of message passing borrows the shared memory mechanism. Compared with message passing, shared memory will have more contention, but there is no need to make multiple copies. In some cases, it is also necessary to consider using this method to deal with it. In Rust, the situation of sharing memory is mainly reflected in the following two aspects:

### static
There are also static variables in the Rust language, whose life cycle is the entire application, and only one instance exists at a fixed address in memory. All threads can access it. This way is also the easiest and most direct way to share. This mechanism exists in almost most languages. Let's take a brief look at the usage of multiple threads accessing static variables in Rust:

```rust
use std::thread;

static VAR: i32 = 5;

fn main() {
	// create a new thread
	let new_thread = thread::spawn(move|| {
	    println!("static value in new thread: {}", VAR);
	});

	// wait for the new thread to run first
	new_thread.join().unwrap();
	println!("static value in main thread: {}", VAR);
}
```

operation result:

```
static value in new thread: 5
static value in main thread: 5
```

`VAR` is a `static` variable that can be used directly in each thread, which is very convenient. Of course, the above is just reading, so it is very simple to modify:

```rust
use std::thread;

static mut VAR: i32 = 5;

fn main() {
    // create a new thread
	let new_thread = thread::spawn(move|| {
	    unsafe {
	    	println!("static value in new thread: {}", VAR);
	    	VAR = VAR + 1;
	    }
	});

    // wait for the new thread to run first
	new_thread.join().unwrap();
	unsafe {
		println!("static value in main thread: {}", VAR);
	}
}
```

operation result:

```
static value in new thread: 5
static value in main thread: 6
```

From the result, the value of `VAR` has changed. From the point of view of the code, in addition to adding the `mut` keyword in front of the `VAR` variable, it is more obvious that `unsafe` is added where `VAR` is used code block. Why? All threads can access `VAR`, and it can be modified, so it is naturally unsafe. The above code is relatively simple, only one thread reads and writes `VAR` at the same time, there will be no problem, so it can be marked with `unsafe`. If there are more threads, please use the synchronization mechanism to be introduced next to handle them.

static is so, what about const? const will be inlined into the code at compile time, so it will not exist at a fixed memory address, nor can it be modified, and it is not memory shared.

### heap
Due to the design of modern operating systems, threads are parasitic to processes and can share process resources. If you want to share a variable among threads, then in addition to the above static, there is another way to save the variable on the heap. Of course, Rust is no exception and follows this design. It's just that we know that Rust will definitely do some considerations in terms of security, so that the language design and use are slightly different.

In order to allocate space on the heap, Rust provides `std::boxed::Box`. Due to the characteristics of the heap, the survival time is relatively long, so in addition to the sharing between threads introduced here, there are other uses. Here If you don’t understand it in detail, please study or review the introduction of the chapter **Heap, Stack and Box**. Let's take a look at how to access variables created by `Box` across multiple threads:

```rust
use std::thread;
use std::sync::Arc;

fn main() {
	let var : Arc<i32> = Arc::new(5);
	let share_var = var.clone();

	// create a new thread
	let new_thread = thread::spawn(move|| {
		println!("share value in new thread: {}, address: {:p}", share_var, &*share_var);
	});

	// Wait for the new thread to execute first
	// Wait for the new thread to execute first
	new_thread.join().unwrap();
	println!("share value in main thread: {}, address: {:p}", var, &*var);
}
```

operation result:

```
share value in new thread: 5, address: 0x2825070
share value in main thread: 5, address: 0x2825070
```

You may find it very strange, why didn't you see the variable created by Box above, which is obviously the use of `Arc`? If the variables created by `Box` are to be safely used in multiple threads, we need to implement many functions, which need to be `Sync`, and `Arc` is a shared reference counting implemented by `Box` The wrapper class for the state. The following quotes a piece of `Arc::new` source code to see that it is implemented through `Box`:

```rust
pub fn new(data: T) -> Arc<T> {
    // Start the weak pointer count as 1 which is the weak pointer that's
    // held by all the strong pointers (kinda), see std/rc.rs for more info
    let x: Box<_> = box ArcInner {
        strong: atomic::AtomicUsize::new(1),
        weak: atomic::AtomicUsize::new(1),
        data: data,
    };
    Arc { _ptr: unsafe { NonZero::new(Box::into_raw(x)) } }
}
```

Through the above running results, we can also find that the `address` printed in the new thread and the main thread is the same, indicating that the state is indeed at the same memory address.

If the resources allocated by `Box` on the heap are only used in one thread, it is very simple to release them. Just release them in time after using them. If it is to be used in multiple threads, you need to face two key problems:

1. When are resources released?
2. How can threads safely modify and read concurrently?

Due to the existence of the above two problems, this is why we cannot directly use the `Box` variable to share among threads. It can be seen that shared memory seems to be much more complicated than the message passing mechanism. Rust uses reference counting to solve the first problem. It provides two wrapper classes in the standard library. In addition to the above one for multi-threading `std::sync::Arc`, there is another one that cannot be used `std::rc::Rc` for multithreading. When using, you can choose according to your needs. If you accidentally use `std::rc::Rc` with multiple threads, the compiler will correct you mercilessly.

Regarding the second question above, the Rust language and standard library provide a series of synchronization methods to solve it. In the following chapters, we will explain these methods and usage in detail.
