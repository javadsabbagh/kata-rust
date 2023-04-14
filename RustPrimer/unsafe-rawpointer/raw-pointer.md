# raw pointer

**Rust** ensures compile-time safety by restricting the behavior of smart pointers, but still requires some additional operations on pointers.

`*const T` and `*mut T` are called "naked pointers" in **Rust**. It allows aliasing, allows writing to types with shared ownership, and even memory-safe shared memory types such as `Rc<T>` and `Arc<T>`, but giving you more power also means that you need to be responsible Additional responsibilities:

* is not guaranteed to point to valid memory, or even non-null
* There is no automatic cleanup, so resources need to be managed manually
* is a plain old type, i.e. it does not move ownership, so **Rust** compilers cannot guarantee against bugs like use-after-free
* lacks any form of lifetime, unlike `&`, so the compiler cannot detect dangling pointers
* There are no aliases or mutability guarantees, except that direct changes via `*const T` are not allowed

## use

Create a raw pointer:

```rust
let a = 1;
let b = &a as *const i32;

let mut x = 2;
let y = &mut x as *mut i32;
```

Dereferencing needs to be done in `unsafe`:

```rust
let a = 1;
let b = &a as *const i32;
let c = unsafe { *b };
println!("{}", c);
```

`into_raw` for `Box<T>`:

```rust
let a: Box<i32> = Box::new(10);
// 我们需要先解引用a，再隐式把 & 转换成 *
let b: *const i32 = &*a;
// 使用 into_raw 方法
let c: *const i32 = Box::into_raw(a);
```

As mentioned above, implicit conversions can be made between references and raw pointers, 
but dereferencing after implicit conversions requires the use of `unsafe`:

```rust
// explicit
let a = 1;
let b: *const i32 = &a as *const i32; // or let b = &a as *const i32;
// Implicit
let c: *const i32 = &a;
unsafe {
	println!("{}", *c);
}

```
