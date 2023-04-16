# trait object (trait object)

A trait object in **Rust** refers to a trait encapsulated by a pointer, such as `&SomeTrait` and `Box<SomeTrait>`.

```rust
trait Foo { fn method(&self) -> String; }

impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }

fn do_something(x: &Foo) {
    x.method();
}

fn main() {
    let x = "Hello".to_string();
    do_something(&x);
    let y = 8u8;
    do_something(&y);
}
```

`x: &Foo` where `x` is a trait object, the pointer is used here because `x` can be any type instance that implements `Foo`, the memory size is not determined, but the size of the pointer is fixed.

## Implementation of trait object

The `&SomeTrait` type is different from the normal pointer type `&i32`. It includes not only pointers to real objects, but also a pointer to a virtual function table. Its internal implementation is defined in the `std::raw` module:

```rust
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}
```

Where `data` is a pointer to an instance of the actual type, and `vtable` is a virtual function table pointing to the actual type's implementation of the trait:

The vtable type of `Foo`:

```rust
struct FooVtable {
    destructor: fn(*mut ()),
    size: usize,
    align: usize,
    method: fn(*const ()) -> String,
}
```

The previous code can be read as:

```rust
//u8:
// This function will only be called with a pointer to u8
fn call_method_on_u8(x: *const ()) -> String {
    let byte: &u8 = unsafe { &*(x as *const u8) };

    byte.method()
}

static Foo_for_u8_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    size: 1,
    align: 1,

    method: call_method_on_u8 as fn(*const ()) -> String,
};


// String:
// This function will only be called with a pointer to String
fn call_method_on_String(x: *const ()) -> String {
    let string: &String = unsafe { &*(x as *const String) };

    string.method()
}

static Foo_for_String_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    size: 24,
    align: 8,

    method: call_method_on_String as fn(*const ()) -> String,
};


let a: String = "foo".to_string();
let x: u8 = 1;

// let b: &Foo = &a;
let b = TraitObject {
     // data stores a reference to the actual value
     data: &a,
     // vtable stores the actual type to implement the method of Foo
     vtable: &Foo_for_String_vtable
};

// let y: &Foo = x;
let y = TraitObject {
    data: &x,
    vtable: &Foo_for_u8_vtable
};

// b.method();
(b.vtable.method)(b.data);

// y.method();
(y.vtable.method)(y.data);
```

## Object Security

Not all traits can be used as trait objects, such as:

```rust
let v = vec![1, 2, 3];
let o = &v as &Clone;
```

There will be an error:

```
error: cannot convert to a trait object because trait `core::clone::Clone` is not object-safe [E0038]
let o = &v as &Clone;
        ^~
note: the trait cannot require that `Self : Sized`
let o = &v as &Clone;
        ^~
```
Let me analyze the cause of the error:

```rust
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) { ... }
}
```

Although `Clone` itself inherits the `Sized` trait, its methods `fn clone(&self) -> Self` and `fn clone_from(&mut self, source: &Self) { ... }` contain `Self` type , and **Rust** is dynamically dispatched when using the trait object method, we don’t know the actual type of the trait object at all, it can be any value of any type that implements the trait, so `Self` is here The size of is not `Self: Sized`, this situation is called `object-unsafe` or `not object-safe` in **Rust**, such a trait cannot be a trait object.

Summarize:

If a `trait` method is `object safe`, it needs to satisfy:

* method has a `Self: Sized` constraint, or
* All of the following conditions are met at the same time:
  * no generic parameters
  * is not a static function
  * Other parameters and return values except `self` cannot use `Self` type

If a `trait` is `object-safe`, it needs to satisfy:

* All methods are `object-safe`, and
* trait does not require the `Self: Sized` constraint

Refer to [stackoverflow](http://stackoverflow.com/questions/29985153/trait-object-is-not-object-safe-error)
[object safe rfc](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)
