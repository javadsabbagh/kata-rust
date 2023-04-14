# unsafe

**Rust** The memory safety of C++ relies on a strong type system and compile-time checking, but it is not suitable for all scenarios.
First of all, all programming languages need to deal with external "unsafe" interfaces, call external libraries, etc., which cannot be realized under "safe" Rust; secondly, "safe" Rust cannot efficiently represent complex data structures, Especially when there are various pointers referencing each other inside the data structure; again,
In fact, there are some operations that are safe but cannot be verified by the compiler.

Therefore, behind the safe Rust, the support of `unsafe` is also needed.

Additional things the `unsafe` block allows the programmer to do are:

* Dereference a raw pointer `*const T` and `*mut T`
 
```rust
let x = 5;
let raw = &x as *const i32;
let points_at = unsafe { *raw };
println!("raw points at {}", points_at);
```

* Read and write a mutable static variable `static mut`

```rust
static mut N: i32 = 5;
unsafe {
    N += 1;
    println!("N: {}", N);
}
```

* call an unsafe function

```rust
unsafe fn foo() {
	// implement
}
fn main() {
	unsafe {
    	foo();
    }
}
```

## use `unsafe`

An `unsafe fn` unsafe function flags that calling it may violate **Rust**'s memory safety semantics:

```rust
unsafe fn danger_will_robinson() {
    // implement
}
```

`unsafe block` An unsafe block can call unsafe code within it:

```rust
unsafe {
    // implement
}
```

`unsafe trait`Unsafe traits and their implementations, all concrete types that implement them may be unsafe:

```rust
unsafe trait Scary { }
unsafe impl Scary for i32 {}
```

## safe != no bug

For **Rust**, it is its job to prohibit you from doing any unsafe things, but some are `bugs` when writing code, and they do not belong to the category of "memory safety":

* deadlock
* Out of memory or other resources
* exit uncalled destructor
* integer overflow

There are some special cases to be aware of when using `unsafe`:

* Data race
* Dereference null and dangling raw pointers
* read uninitialized memory
* Use raw pointers to break pointer overlap rules
* `&mut T` and `&T` follow the LLVM-wide `noalias` model, except if `&T` contains an `UnsafeCell<U>`. Unsafe code must not violate these aliasing guarantees
* Do not use `UnsafeCell<U>` to change an immutable value/reference
* Invocation of undefined behavior via compiler intrinsics:
  * Use `std::ptr::offset` (offset function) to index values beyond the bounds of the object, except that the last bit is allowed to exceed one byte
  * Use `std::ptr::copy_nonoverlapping_memory` on overlapping buffers (memcpy32/memcpy64 function)
* Invalid values for primitive types, even in private fields/local variables:
  * Empty/dangling reference or boxed
  * A value in `bool` that is not `false` (0) or `true` (1)
  * one of the `enum` does not contain a discriminant in the type definition
  * A surrogate in `char` or a value exceeding char::MAX
  * non-UTF-8 byte sequence in `str`
* Using Rust in external code or using external languages in Rust



