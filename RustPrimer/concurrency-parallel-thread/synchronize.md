## Synchronize

Synchronization refers to the cooperation between threads to jointly complete a certain task. During the whole process, two key points need to be paid attention to: one is the access of shared resources, and the other is the order of accessing resources. Through the previous introduction, we already know how to allow multiple threads to access shared resources, but we have not introduced how to control the order of access so that no errors will occur. If two threads access the data at the same memory address at the same time, one writes and the other reads, if no control is added, the writing thread only writes half of the data, and the reading thread starts to read, and the data read must be wrong and unusable. Causes program errors, which causes concurrent security issues, for which we must have a set of control mechanisms to avoid such things from happening. It's like two people drinking a bottle of Coke, but there is only one straw, so a rule must be negotiated so that they can all drink Coke in peace. This section will specifically introduce what we need to do in Rust to solve this problem.

Continuing with the above example of drinking coke, the method of drinking one mouthful per person is a solution. As long as you are not too stupid, you can almost think of this solution. In the specific implementation, when A is drinking, B keeps staring at the side. If A takes a sip, B immediately takes it to drink. At this time, A must also be staring at the side. In real life, such examples abound. If you think about it, it seems that waiting may be involved in synchronization. Mr. Zhuge can only wait when everything is ready and the only thing he needs is the east wind, because the conditions are not ripe. According to this logic, almost all operating systems and major programming languages ​​support the current thread waiting, and of course Rust is no exception.

### wait
There is no difference in mechanism between thread waiting in Rust and other languages. There are roughly the following types:

* After waiting for a period of time, continue to execute. It looks like a person is tired from work and takes a break before working again. By calling the relevant API, the current thread can be suspended and enter the sleep state. At this time, the scheduler will not schedule it to execute. After a period of time, the thread will automatically enter the ready state and can be scheduled for execution, and continue to execute from the place where it was sleeping before. . The corresponding APIs are `std::thread::sleep`, `std::thread::sleep_ms`, `std::thread::park_timeout`, `std::thread::park_timeout_ms`, and some similar others Due to too many APIs, please refer to the official website [`std::thread`](https://doc.rust-lang.org/stable/std/thread/index.html) for details.
* This method is a bit special, the time is very short, only one time slice, the current thread actively abandons the scheduling of the current time slice, and let the scheduler re-select the thread to execute, thus giving other threads the opportunity to run, but Note that if other threads have no better reason to execute, of course, the last chance to execute is still it. In the actual application business, for example, after the producer manufactures a product, he can give up a time slice, allowing consumers to get the execution opportunity, so as to quickly consume the newly produced product. This kind of control granularity is very small and needs to be used reasonably. If you need to give up multiple time slices in a row, you can use a loop to implement it. The corresponding API is `std::thread::yield_now`, see the official website [`std::thread`](https://doc.rust-lang.org/stable/std/thread/index.html) for details.
* The waiting of 1 and 2 can continue to execute after a period of time without the assistance of other threads. Finally, we also encountered a kind of waiting, which requires the participation of other threads to wake up the waiting thread, otherwise, the thread will wait forever. Just like a woman, if she has not met a man, she will never be able to get rid of the single state. Related APIs include `std::thread::JoinHandle::join`, `std::thread::park`, `std::sync::Mutex::lock`, etc. There are also some APIs related to synchronization It also causes threads to wait. For details, see the official website [`std::thread`](https://doc.rust-lang.org/stable/std/thread/index.html) and [`std::sync`](https://doc .rust-lang.org/stable/std/sync/index.html).

In fact, we have encountered the first and third waiting methods in the above introduction, and they are also the two most used methods. Here, you can also go back and look at the previous usage methods and effects, and combine your own understanding to do some simple exercises.

Undoubtedly, the third method is a bit more complicated. To wake up the waiting thread, it must be based on certain rules. Don't ring when the time comes. No matter what the rules are based on, to trigger the wake-up event, a certain condition must have been met. Based on this logic, in the operating system and programming language, something called **condition variable** is introduced. It can simulate the behavior of the alarm clock in real life, and the thread waiting for the condition will be notified when the condition is met. Rust's condition variable is `std::sync::Condvar`, see the official website [condition variable](https://doc.rust-lang.org/std/sync/struct.Condvar.html) for details. But notifications are not just a patent of condition variables, there are other ways to trigger notifications, let's take a look below.

### notify
Seeing as a simple notification, you also need to pay attention to the following points when programming:

* Notification must be due to waiting, so notification and waiting almost always appear in pairs, such as `std::sync::Condvar::wait` and `std::sync::Condvar::notify_one`, `std: :sync::Condvar::notify_all`.
* The object used for waiting is the same object used for notification, so this object needs to be shared among multiple threads, see the example below.
* In addition to `Condvar`, *lock* also has an automatic notification function. When the thread holding the lock releases the lock, the thread waiting for the lock will be automatically awakened to seize the lock. The introduction of the lock is explained in detail below.
* Through condition variables and locks, you can also build more complex automatic notification methods, such as `std::sync::Barrier`.
* Notifications can also be 1:1 or 1:N. `Condvar` can control one notification or N notifications, but the lock cannot be controlled. As long as the lock is released, all other threads waiting for the lock will wake up at the same time. Instead of only the first waiting thread.

Let's analyze a simple example:

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {

	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair2 = pair.clone();

	// create a new thread
	thread::spawn(move|| {
	    let &(ref lock, ref cvar) = &*pair2;
	    let mut started = lock.lock().unwrap();
	    *started = true;
	    cvar.notify_one();
	    println!("notify main thread");
	});

	// wait for the new thread to run first
	let &(ref lock, ref cvar) = &*pair;
	let mut started = lock.lock().unwrap();
	while !*started {
		println!("before wait");
	    started = cvar.wait(started).unwrap();
	    println!("after wait");
	}
}
```

operation result:

```
before waiting
notify main thread
after waiting
```

This example shows how to control the synchronization between the newly created thread and the main thread through condition variables and locks, so that the main thread can continue to execute after waiting for the newly created thread to execute. Judging from the results, the function is realized. For the above example, the following points need to be explained:

* `Mutex` is a type of lock in Rust.
* `Condvar` needs to be used together with `Mutex`, because `Mutex` is protected, and `Condvar` is safe to be concurrent.
* The `Mutex::lock` method returns a `MutexGuard`, which is automatically destroyed when it leaves the scope, thereby automatically releasing the lock, thereby avoiding the problem that the lock is not released.
* `Condvar` will release the lock when it is waiting, and will reacquire the lock when it is notified to wake up, so as to ensure the safety of concurrency.

At this point, you should be more interested in locks, why do you need locks? The purpose of the lock is to ensure that resources can be accessed in an orderly manner at the same time without abnormal data. But in fact, to achieve this, there are not only locks, including locks, which mainly involve two basic methods:

### Atom Types
Atomic type is the simplest mechanism to control access to shared resources. Compared with the locks that will be introduced later, atomic type does not require developers to deal with the problem of locking and releasing locks, and supports operations such as modification and reading. It also has high concurrency performance, and basically supports everything from hardware to operating systems to various languages. In the standard library `std::sync::atomic`, you will see Rust's existing atomic types in it, including `AtomicBool`, `AtomicIsize`, `AtomicPtr`, `AtomicUsize`. These four atomic types can basically satisfy 90% of the security access needs of shared resources. Next, we will use the atomic type, combined with the knowledge of shared memory, to show how one thread modifies and one thread reads:

```rust
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
	let var : Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));
	let share_var = var.clone();

	// create a new thread
	let new_thread = thread::spawn(move|| {
		println!("share value in new thread: {}", share_var.load(Ordering::SeqCst));
		// Modify the value
		share_var.store(9, Ordering::SeqCst);
	});

	// Wait for the new thread to execute first
	new_thread.join().unwrap();
	println!("share value in main thread: {}", var.load(Ordering::SeqCst));
}
```

operation result:

```
share value in new thread: 5
share value in main thread: 9
```

The result shows that the new thread successfully modified the value and got the latest value in the main thread. You can also try to use other atomic types. Here we can think about it, if we use `Arc::new(*mut Box<u32>)`, can it be done? Why? After thinking about it, you will realize how good Rust is in terms of multi-thread safety. In addition to atomic types, we can also use locks to achieve the same function.

### Lock
To share resources in multithreading, in addition to atomic types, locks can also be considered. The lock must be obtained before the operation. A lock can only be given to one thread at the same time, so that only one thread can operate the shared resource at the same time. After the operation is completed, the lock is released to other waiting threads. In Rust `std::sync::Mutex` is a kind of lock. Let's use `Mutex` to implement an example of the above atomic type:

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
	let var : Arc<Mutex<u32>> = Arc::new(Mutex::new(5));
	let share_var = var.clone();

	// Create a new thread
	let new_thread = thread::spawn(move|| {
		let mut val = share_var.lock().unwrap();
		println!("share value in new thread: {}", *val);
		// Modify the value
		*val = 9;
	});

	//Wait for the new thread to execute first
	new_thread.join().unwrap();
	println!("share value in main thread: {}", *(var.lock().unwrap()));
}
```

operation result:

```
share value in new thread: 5
share value in main thread: 9
```

The results are the same, it seems that `Mutex` can also be used, but if compared in terms of efficiency, the atomic type will be even better. Regardless of this point, we can see from the code that although there is `lock`, there is no code similar to `unlock`. It is not that there is no need to release the lock, but that Rust has already When `val` is destroyed, the lock is automatically released. At the same time, we found that in order to modify the shared value, developers must call `lock`, which solves another security problem. I have to praise Rust again for its safety in multi-threading. If it is another language, if we want to be safe, we must implement these ourselves.

In order to ensure the safety of locks, Rust has done a lot of work, but in terms of efficiency, it is not as good as atomic types. So is the lock worthless? Obviously the fact cannot be like this, since it exists, it must have its value. It can solve the ten percent of problems that atomic type locks cannot solve. Let's look at the previous example again:

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {

	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair2 = pair.clone();

	// create a new thread
	thread::spawn(move|| {
	    let &(ref lock, ref cvar) = &*pair2;
	    let mut started = lock.lock().unwrap();
	    *started = true;
	    cvar.notify_one();
	    println!("notify main thread");
	});

	// wait for the new thread to run first
	let &(ref lock, ref cvar) = &*pair;
	let mut started = lock.lock().unwrap();
	while !*started {
		println!("before wait");
	    started = cvar.wait(started).unwrap();
	    println!("after wait");
	}
}
```

The `Condvar` in the code is the condition variable, which provides the `wait` method to actively make the current thread wait, and also provides the `notify_one` method to allow other threads to wake up the waiting thread. In this way, the sequence control can be perfectly realized. It looks like the condition variable does all the work, so what's the need for `Mutex`? In order to prevent multiple threads from executing the `wait` operation of the condition variable at the same time, because the condition variable itself also needs to be protected. This is what the lock can do, but the atomic type cannot.

In Rust, a `Mutex` is an exclusive lock that only one thread can hold at a time. This kind of lock will cause all threads to be serialized, which guarantees safety but is not efficient. For the case of writing less and reading more, if there is no writing, it is all read, then it should be executed concurrently. In order to achieve this goal, almost all programming languages ​​provide a method called read-write lock. The mechanism also exists in Rust, called [`std::sync::RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html), the use is the same as `Mutex `Almost, I will leave it to everyone to practice on their own.

Synchronization is an eternal theme of multi-threaded programming. Rust has provided us with a good programming paradigm and imposed checks. Even if you have not had much contact with it before, you can write very safe multi-threaded programs with Rust.
