# Compile Rust into a library
The previous chapter described how to call the c library from rust. In this chapter, we will talk about how to compile rust into a library for other languages to call through cffi.

## Calling convention and mangle
As mentioned in the previous chapter, in order to allow rust functions to be called through ffi, it is necessary to add `extern "C"` to modify the function.

But since rust supports overloading, the function name will be confused by the compiler, just like c++. So when your function is compiled, the function name will be accompanied by a string indicating the function signature.

For example: `fn test() {}` will become `_ZN4test20hf06ae59e934e5641haaE`.
Such a function called ffi brings difficulties, therefore, rust provides the `#[no_mangle]` attribute to decorate the function.
For functions with the `#[no_mangle]` attribute, the rust compiler will not perform function name mangling for it. like:

```rust
#[no_mangle]
extern "C" fn test() {}
```

Observed in nm as

```
...
00000000001a7820 T test
...
```

So far, `test` function will be able to be called by `cffi` normally.
## Specify `crate` type
`rustc` compiles and generates the `rlib` format library used by `rust` by default. To make `rustc` generate a dynamic link library or a static link library, it needs to be explicitly specified.

1. Method 1: Specify in the file.
    Add `#![crate_type = "foo"]` to the file header, where the optional types of `foo` are `bin`, `lib`, `rlib`, `dylib`, `staticlib`, corresponding to executable document,
    Default (will be determined by `rustc`), `rlib` format, dynamic link library, static link library.
2. Method 2: Pass `--crate-type` parameter to rustc when compiling. The parameter content is the same as above.
3. Method 3: Use cargo, specify `crate-type = ["foo"] `, `foo` optional type is the same as 1

## Tip: `Any`

Because the type information of `rust` will be lost in the process of crossing `ffi`, for example, when using `rust` to provide an `OpaqueStruct` to other languages:

```rust
use std::mem::transmute;

#[derive(Debug)]
struct Foo<T> {
  t: T
}

#[no_mangle]
extern "C" fn new_foo_vec() -> *const c_void {
    Box::into_raw(Box::new(Foo {t: vec![1,2,3]})) as *const c_void
}

#[no_mangle]
extern "C" fn new_foo_int() -> *const c_void {
    Box::into_raw(Box::new(Foo {t: 1})) as *const c_void
}

fn push_foo_element(t: &mut Foo<Vec<i32>>) {
    t.t.push(1);
}

#[no_mangle]
extern "C" fn push_foo_element_c(foo: *mut c_void){
    let foo2 = unsafe {
        &mut *(foo as *mut Foo<Vec<i32>>) // So sure it is Foo<Vec<i32>>? What if foo is Foo<i32>?
    };
    push_foo_element(foo3);
}
```

The code above has absolutely no idea what `foo` is. There is no way to talk about security, you can only rely on documents.
Therefore, the convenience and safety brought by the `rust` type system are often lost when calling `ffi`. Here's a little trick: use `Box<Box<Any>>` to wrap your types.

The `Any` type of `rust` brings runtime reflection capabilities to `rust`, and using `Any` to cross the `ffi` boundary will greatly improve program security.

```rust
use std::any::Any;

#[derive(Debug)]
struct Foo<T> {
  t: T
}

#[no_mangle]
extern "C" fn new_foo_vec() -> *const c_void {
    Box::into_raw(Box::new(Box::new(Foo {t: vec![1,2,3]}) as Box<Any>)) as *const c_void
}

#[no_mangle]
extern "C" fn new_foo_int() -> *const c_void {
    Box::into_raw(Box::new(Box::new(Foo {t: 1}) as Box<Any>)) as *const c_void
}

fn push_foo_element(t: &mut Foo<Vec<i32>>) {
    t.t.push(1);
}

#[no_mangle]
extern "C" fn push_foo_element_c(foo: *mut c_void){
    let foo2 = unsafe {
        &mut *(foo as *mut Box<Any>)
    };
    let foo3: Option<&mut Foo<Vec<i32>>> = foo2.downcast_mut(); // if foo2 is not *const Box<Foo<Vec<i32>>>, then foo3 will be None
    if let Some(value) = foo3 {
      push_foo_element(value);
    }
}
```

In this way, it is very easy to make mistakes.
