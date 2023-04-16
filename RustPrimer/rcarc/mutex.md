# Mutex and RwLock

## Mutex

`Mutex` means mutual exclusion object, used to protect shared data. Mutex has the following characteristics:

1. `Mutex` will wait to acquire the lock token (token), and will block the thread during the waiting process. until the lock token is obtained. At the same time, only one thread's `Mutex` object acquires the lock;
2. `Mutex` tries to get the lock token through `.lock()` or `.try_lock()`. The protected object must be called through the `RAII` guard returned by these two methods, and cannot be directly operated;
3. When the guard scope of `RAII` ends, the lock will be unlocked automatically;
4. In multithreading, `Mutex` is generally used in conjunction with `Arc`.

Example:

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

const N: usize = 10;

// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.
let data = Arc::new(Mutex::new(0));

let (tx, rx) = channel();
for _ in 0..10 {
     let (data, tx) = (data. clone(), tx. clone());
     thread::spawn(move || {
         // The shared state can only be accessed once the lock is held.
         // Our non-atomic increment is safe because we're the only thread
         // which can access the shared state when the lock is held.
         //
         // We unwrap() the return value to assert that we are not expecting
         // threads to ever fail while holding the lock.
         let mut data = data. lock(). unwrap();
         *data += 1;
         if *data == N {
             tx. send(()). unwrap();
         }
         // the lock is unlocked here when `data` goes out of scope.
     });
}

rx.recv().unwrap();
```

### Difference between `lock` and `try_lock`

The `.lock()` method will wait for the lock token, and when waiting, it will block the current thread. The `.try_lock()` method is just a trial operation and will not block the current thread.

When `.try_lock()` does not get a lock token, `Err` will be returned. Therefore, if you want to use `.try_lock()`, you need to do careful handling of the return value (for example, in a loop check).


__Comment__: Rust's Mutex is designed as an object, which is different from the implementation of the spin lock in C language with two separate statements, which is safer, more beautiful, and easier to manage.


## RwLock

`RwLock` translates to `read-write lock`. It is characterized by:

1. Multiple reads are allowed at the same time, but only one write at most;
2. Reading and writing cannot exist at the same time;

for example:

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);

// many reader locks can be held at once
{
     let r1 = lock. read(). unwrap();
     let r2 = lock. read(). unwrap();
     assert_eq!(*r1, 5);
     assert_eq!(*r2, 5);
} // read locks are dropped at this point

// only one write lock may be held, however
{
     let mut w = lock.write().unwrap();
     *w += 1;
     assert_eq!(*w, 6);
} // write lock is dropped here
```

### Read-write lock method

1. `.read()`
2. `.try_read()`
3. `.write()`
4. `.try_write()`

Note that you need to judge the return values of `.try_read()` and `.try_write()`.