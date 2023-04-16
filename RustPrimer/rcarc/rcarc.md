# Rc and Arc

Rust's set of mechanisms based on ownership requires that a resource has and can only have one binding or `&mut` reference with ownership at the same time, which ensures memory safety in most cases. But this design is quite strict. In other cases, it restricts the writing of programs and cannot realize certain functions. Therefore, Rust provides additional measures in the std library to supplement the ownership mechanism to deal with a wider range of scenarios.

By default, in Rust, there is one and only one owner of a resource at the same time. `Rc` and `Arc` use the method of reference counting to allow the program to realize multiple owners of the same resource at the same time, and multiple owners share the resource.

## Rc
`Rc` is used inside the same thread, introduced by `use std::rc::Rc`. It has the following characteristics:

1. The type object wrapped with `Rc` is `immutable`, that is, unchangeable. That is, you cannot modify the `T` object in `Rc<T>`, you can only read it;
2. Once the last owner disappears, the resource will be automatically reclaimed, and this life cycle is determined at compile time;
3. `Rc` can only be used within the same thread, and cannot be used for object sharing between threads (cannot be passed across threads);
4. `Rc` is actually a pointer, which does not affect the method call form of the wrapped object (that is, there is no such thing as unpacking and then calling the value).

example:

```rust
use std::rc::Rc;

let five = Rc::new(5);
let five2 = five.clone();
let five3 = five.clone();

```

## Rc Weak

`Weak` is introduced by `use std::rc::Weak`.

`Rc` is a reference count pointer, and `Weak` is a pointer, but does not increase the reference count, which is the weak version of `Rc`. It has the following characteristics:

1. Accessible, but not owned. Does not increase the reference count, therefore, will not affect resource recovery management;
2. It can be converted to `Weak<T>` by calling the `downgrade` method from `Rc<T>`;
3. `Weak<T>` can be converted to `Option<Rc<T>>` using the `upgrade` method. If the resource has been released, the Option value is `None`;
4. It is often used to solve the problem of circular references.

example:

```rust
use std::rc::Rc;

let five = Rc::new(5);

let weak_five = Rc::downgrade(&five);

let strong_five: Option<Rc<_>> = weak_five.upgrade();
```

## Arc

`Arc` is atomic reference counting and is a multi-threaded version of `Rc`. `Arc` is introduced via `std::sync::Arc`.

Its features:

1. `Arc` can be passed across threads, used to share an object across threads;
2. A type object wrapped with `Arc` has no requirement for mutability;
3. Once the last owner disappears, the resource will be automatically reclaimed, and this life cycle is determined at compile time;
4. `Arc` is actually a pointer, which does not affect the method call form of the wrapped object (that is, there is no such thing as unpacking and then calling the value);
5. `Arc` is **almost necessary** for multi-threaded shared state (reduce copying, improve performance).

Example:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);

    for _ in 0..10 {
        let child_numbers = shared_numbers.clone();

        thread::spawn(move || {
            let local_numbers = &child_numbers[..];

            // Work with the local numbers
        });
    }
}
```

### Arc Weak

Similar to `Rc`, `Arc` also has a corresponding `Weak` type, imported from `std::sync::Weak`.

The meaning and usage are basically the same as `Rc Weak`, the difference is that this is a multi-threaded version. So no more details.



## one example

The following example shows how to implement multiple objects to refer to another object at the same time.

```rust
use std::rc::Rc;

struct Owner {
    name: String
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>
}

fn main() {
    // Create a reference counted Owner.
    let gadget_owner : Rc<Owner> = Rc::new(
        Owner { name: String::from("Gadget Man") }
    );

    // Create Gadgets belonging to gadget_owner. To increment the reference
    // count we clone the `Rc<T>` object.
    let gadget1 = Gadget { id: 1, owner: gadget_owner.clone() };
    let gadget2 = Gadget { id: 2, owner: gadget_owner.clone() };

    drop(gadget_owner);

    // Despite dropping gadget_owner, we're still able to print out the name
    // of the Owner of the Gadgets. This is because we've only dropped the
    // reference count object, not the Owner it wraps. As long as there are
    // other `Rc<T>` objects pointing at the same Owner, it will remain
    // allocated. Notice that the `Rc<T>` wrapper around Gadget.owner gets
    // automatically dereferenced for us.
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // At the end of the method, gadget1 and gadget2 get destroyed, and with
    // them the last counted references to our Owner. Gadget Man now gets
    // destroyed as well.
}
```
