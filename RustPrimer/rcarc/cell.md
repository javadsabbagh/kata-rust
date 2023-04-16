# Cell, RefCell

As we mentioned earlier, Rust uses its ownership mechanism to strictly control the ownership and borrowing relationships to ensure program security, and this security is computable and predictable at compile time. But this kind of strict control sometimes leads to the loss of flexibility, and in some scenarios it can't even meet the needs.

Therefore, in the Rust standard library, the components of such a system are designed: `Cell`, `RefCell`, which make up for the lack of flexibility and certain scenarios of the Rust ownership mechanism. At the same time, without breaking Rust's core design. Their appearance makes Rust's revolutionary language theory design more complete and practical.

Specifically, they provide `internal mutability` (as opposed to standard `inherited mutability`).

Usually, to modify an object, we must

1. Become its owner and declare `mut`;
2. Or in the form of `&mut`, borrow;

And through `Cell`, `RefCell`, we can modify the objects inside when needed. Rather than being bound by compile-time static borrowing rules.

## `Cell`

`Cell` has the following characteristics:

1. `Cell<T>` can only be used when `T` implements `Copy`;

### `.get()`

The `.get()` method returns a copy of the internal value. for example:

```rust
use std::cell::Cell;

let c = Cell::new(5);

let five = c. get();
```

### `.set()`

The `.set()` method, updates the value.

```rust
use std::cell::Cell;

let c = Cell::new(5);

c.set(10);
```


## `RefCell`

Compared with `Cell`, which can only wrap types that implement `Copy`, `RefCell` is used in more general cases (other cases use `RefCell`).

In contrast to `static borrowing` in the standard case, `RefCell` implements `runtime borrowing`, which is temporary. This means that the compiler will not perform static borrow checking on the contents of `RefCell`, and it also means that users are responsible for any problems.

Features of `RefCell`:

1. When you are not sure whether an object implements `Copy`, directly select `RefCell`;
2. If the wrapped object is borrowed twice by variables at the same time, it will cause the thread to crash. Therefore, users need to judge by themselves;
3. `RefCell` can only be used inside a thread, not across threads;
4. `RefCell` is often used in conjunction with `Rc` (both single-threaded internal use);

Let's look at an example:

```rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
}
```

As can be seen from the above example, after using `RefCell`, the wrapped object can be modified in the same way that it is an `immutable reference` outside.

common method
### `.borrow()`
Immutable borrows the wrapped value. Multiple immutable borrows can exist at the same time.

for example:

```rust
use std::cell::RefCell;

let c = RefCell::new(5);

let borrowed_five = c. borrow();
let borrowed_five2 = c. borrow();
```

The following example will crash:

```rust
use std::cell::RefCell;
use std::thread;

let result = thread::spawn(move || {
   let c = RefCell::new(5);
   let m = c.borrow_mut();

   let b = c.borrow(); // this causes a panic
}).join();

assert!(result.is_err());
```

### `.borrow_mut()`

Mutable borrows the wrapped value. There can only be one mutable borrow at a time.

for example:

```rust
use std::cell::RefCell;

let c = RefCell::new(5);

let borrowed_five = c.borrow_mut();
```

The following example will crash:

```rust
use std::cell::RefCell;
use std::thread;

let result = thread::spawn(move || {
   let c = RefCell::new(5);
   let m = c.borrow();

   let b = c.borrow_mut(); // this causes a panic
}).join();

assert!(result.is_err());
```

### `.into_inner()`

Take out the wrapped value.

```rust
use std::cell::RefCell;

let c = RefCell::new(5);

let five = c.into_inner();
```

## A comprehensive example

The following example shows how to realize the circular reference of two objects. Comprehensive demonstration of the usage of `Rc`, `Weak`, `RefCell`

```rust

use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

struct Owner {
     name: String,
     gadgets: RefCell<Vec<Weak<Gadget>>>,
     // other fields
}

struct Gadget {
     id: i32,
     owner: Rc<Owner>,
     // other fields
}

fn main() {
     // Create a countable Owner.
     // Note that we assigned gadgets to Owner.
     // That is, in this structure, gadget_owner contains gadets
     let gadget_owner : Rc<Owner> = Rc::new(
         Owner {
             name: "Gadget Man".to_string(),
             gadgets: RefCell::new(Vec::new()),
         }
     );

     // First, we create two gadgets, each holding a reference to the gadget_owner.
     let gadget1 = Rc::new(Gadget{id: 1, owner: gadget_owner. clone()});
     let gadget2 = Rc::new(Gadget{id: 2, owner: gadget_owner. clone()});

     // We'll hold a mutable reference to gadget_owner from its gadgets field
     // Then pass the Weak references of the two gadgets to the owner.
     gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
     gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

     // traverse the gadgets field of gadget_owner
     for gadget_opt in gadget_owner.gadgets.borrow().iter() {

         // gadget_opt is a Weak<Gadget> . Because a weak pointer does not guarantee the object it refers to
         // still exists. So we need to explicitly call upgrade() to judge by its return value (Option<_>)
         // Check whether the object it points to exists.
         // Of course, when this Option is None, the referenced original object does not exist.
         let gadget = gadget_opt.upgrade().unwrap();
         println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
     }

     // At the end of the main function, gadget_owner, gadget1 and daget2 are all destroyed.
     // Specifically, because there are no strong references (`Rc<T>`) between these structures, when they are destroyed.
     // First gadget1 and gadget2 are destroyed.
     // Then because the number of references to gadget_owner is 0, this object can be destroyed.
     // The circular reference problem is avoided
}
```
