# Send and Sync

In the `std::marker` module, there are two traits: `Send` and `Sync`, which are related to multi-thread safety.

A trait marked `marker trait` is actually a convention, with no method definitions or associated items. It's just a convention that types that implement it must satisfy. Whether a type adds this kind of convention is either a compiler behavior or a manual behavior.

`Send` and `Sync` are automatically deduced by the compiler in most cases (for Rust's primitive types and most types in std). For types that cannot be automatically deduced by the compiler, to make them have the contract of `Send` or `Sync`, it can be implemented manually by humans. When implementing, you must use the `unsafe` prefix, because Rust does not trust programmers by default, and all things controlled by programmers are marked as `unsafe`, and something goes wrong (for example, adding `Sync ` convention) is the responsibility of the programmer.

They are defined as follows:

If `T: Send`, passing `T` to another thread (by value) will not cause data races or other unsafe conditions.

1. `Send` means that the object can be safely sent to another executable;
2. `Send` enables the object to be sent to be decoupled from the thread that generated it, preventing the resource from being used in the target thread after the original thread releases it (use after free).

If `T: Sync`, passing `&T` to another thread will not cause data races or other unsafe conditions.

1. `Sync` can be accessed by multiple execution bodies at the same time without error;
2. `Sync` prevents competition;

inference:

1. `T: Sync` means `&T: Send`;
3. `Sync + Copy = Send`;
4. When `T: Send`, `&mut T: Send` can be deduced;
4. When `T: Sync`, `&mut T: Sync` can be deduced;
5. When `&mut T: Send`, `T: Send` cannot be deduced;

(Note: `T`, `&T`, `&mut T`, `Box<T>`, etc. are all different types)


Specific types:

1. Primitive types (for example: u8, f64), are both `Sync` and `Copy`, so they are all `Send`;
2. Composite types that only contain primitive types are `Sync` and `Copy`, so they are `Send`;
3. When `T: Sync`, `Box<T>`, `Vec<T>` and other collection types are `Sync`;
4. Pointers with internal mutability, not `Sync`, such as `Cell`, `RefCell`, `UnsafeCell`;
5. `Rc` is not `Sync`. Because as long as the `&Rc<T>` operation is performed, a new reference will be cloned, which will modify the reference count in a non-atomic way, so it is not safe;
6. The type `T: Send` locked by `Mutex` and `RWLock` is `Sync`;
7. Raw pointers (`*mut`, `*const`) are neither `Send` nor `Sync`;


Rust provides a safe and reliable infrastructure for concurrent programming through these two weapons: `ownership and life cycle` + `Send and Sync` (essentially a type system). So that programmers can rest assured to build a robust concurrency model on it. This is also the embodiment of Rust's core design concept: the kernel only provides the most basic primitives, and the real implementation can be separated if it can be separated. The same goes for concurrency.
