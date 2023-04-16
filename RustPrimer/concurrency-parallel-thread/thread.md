# Concurrent programming
What is concurrency? To quote Rob Pike's classic description:
> Concurrency is the ability to deal with multiple things at the same time

In fact, there are many concurrent things around us, such as sending text messages while attending class; feeding a child while watching TV, as long as you pay attention, you will find many similar things. Correspondingly, in the world of software, we will also find such things, such as writing blogs while listening to music; reading web pages while downloading software, and so on. Obviously this will save a lot of time and do more things. However, at the beginning, the computer system could not handle two things at the same time, which obviously could not meet our needs. Later, a multi-process and multi-thread solution was gradually proposed. Later, the hardware also developed to the point of multi-core and multi-CPU. There is also more and more support for concurrency at the bottom of the hardware and system. Correspondingly, major programming languages also provide strong support for concurrent processing. As an emerging language, Rust naturally supports concurrent programming. Then this chapter will lead you to an overview of the relevant knowledge of Rust concurrent programming, starting from threads, gradually trying to perform data interaction, synchronous collaboration, and finally enter parallel implementation, and uncover the mystery of Rust concurrent programming step by step. Since this book mainly introduces the use of the Rust language, this chapter will not conduct a comprehensive and in-depth discussion on the theoretical knowledge related to concurrent programming - if that is the case, a book is not enough to introduce, but more focused on the introduction How to implement basic concurrency in Rust language.

First, we will introduce the use of threads. Threads are the basic execution unit, and their importance is self-evident. Rust programs are composed of a bunch of threads. In today's situation where multi-core and multi-CPU have become popular, various big data analysis and parallel computing make threads glow more dazzlingly. If you don't know much about threads, please refer to books related to the operating system first, and I won't introduce too much here. It then introduces some implementations of data transfer and collaboration that need to be dealt with when solving concurrency problems, such as message passing, synchronization, and shared memory. Finally, a brief introduction to the implementation of parallelism in Rust.

## 24.1 Thread creation and termination
I believe that threads are not unfamiliar to everyone. With the popularity of multi-CPU and multi-core today, big data analysis and parallel computing are inseparable from it. Almost all languages ​​support it, and all processes are run by a or multiple threads. Since it is so important, let's take a look at how to create a thread in Rust, and then how the thread ends.

Rust's support for threads, like `C++11`, is implemented in the standard library. For details, please refer to [`std::thread`](https://doc.rust-lang.org/ std/thread/index.html), fortunately, Rust has done this from the beginning, so you don't have to wait like C++. With support at the language level, developers don't have to deal with the porting of each platform so hard. From the source code of Rust, we can see that `std::thread` is actually the encapsulation of thread operations on different platforms, and the implementation of related APIs is implemented by calling the API of the operating system, thus providing a unified interface for thread operations. For me, being able to operate native threads in such a simple and quick way, the pressure on me has been relieved a lot.

### Create thread
First, let's take a look at how to create a native thread in Rust. `std::thread` provides two creation methods, both of which are very simple. The first method is created by the `spawn` function, see the sample code below:

```rust
use std::thread;

fn main() {
	// Create a thread
    let new_thread = thread::spawn(move || {
        println!("I am a new thread.");
    });
    // Wait for the newly created thread to execute
    new_thread.join().unwrap();
}
```

Execute the above code and you will see the following output:

```
I am a new thread.
```

Just 5 lines of code, can't be less, the most critical is of course the line of code that calls the `spawn` function. To use this function, remember to `use std::thread` first. Note that the `spawn` function requires a function as a parameter, and it is of type `FnOnce`. If you have forgotten this type of function, please learn or review the function and closure chapter. Even if the last line of code of the `main` function is unnecessary, it can still create a thread (the function and use of the `join` function will be explained in detail in the subsequent sections, here you only need to know that it can be used to wait for the thread to complete execution), you can remove or comment Try this line of code. In this case, the running result may not have any output, and the specific reasons will be explained in detail later.

Next, we use the second method to create a thread, which is a little more complicated than the first method, because it is more powerful, and the thread name and stack size can be set before creation, see the following code:

```rust
use std::thread;

fn main() {
	// Create a thread, the thread name is thread1, and the stack size is 4k
    let new_thread_result = thread::Builder::new()
    						.name("thread1".to_string())
    						.stack_size(4*1024*1024).spawn(move || {
        println!("I am thread1.");
    });
    // Wait for the newly created thread to complete
    new_thread_result.unwrap().join().unwrap();
}
```
Execute the above code and you will see the following output:

```
I am thread1.
```

By comparing the implementation code with the first method, we can find that this method uses a `Builder` class to set the thread name and stack size. In addition, the return value of the `spawn` function of `Builder` is a` Result`, in formal code writing, you can’t directly `unwrap.join` like the above, you should make a judgment. There will be a lot of similar demo codes later, for the sake of simplicity, it will not be very rigorous.

The above are two different ways of creating native threads in Rust. The sample code is a bit of a coincidence, but you can modify it a little to make it more useful. Try it.

### end of thread
At this point, we already know how to create a new thread. After creation, whether you see it or not, it is there, so when will it die? Fend for itself, or be killed? If you have been exposed to some system programming, you should know that some operating systems provide an interface to kill threads roughly. If you find it uncomfortable, just kill it directly, and you can completely ignore the feeling of creating a new thread. It feels cool, but Rust won't make it so cool anymore, because `std::thread` doesn't provide such an interface, why? If you have in-depth contact with concurrent programming or multi-threaded programming, you will know that forcibly terminating a running thread will cause many problems. For example, resources are not released, causing state confusion and unpredictable results. At the moment of forcible killing, it seems that the problem has been solved very happily, but there may be endless troubles. A major feature of the Rust language is safety, and such irresponsible practices are absolutely not allowed. Even if similar interfaces are provided in other languages, they should not be abused.

So in Rust, can the newly created thread only let itself self-destruct? In fact, there are two ways. First, we will introduce the self-defeating method that everyone knows. After the execution of the thread execution body is completed, the thread ends. For example, in the first way of creating a thread above, the code is finished after executing `println!("I am a new thread.");`. If something like this:

```rust
use std::thread;

fn main() {
    // create a thread
    let new_thread = thread::spawn(move || {
        loop {
            println!("I am a new thread.");
        }
    });
    // Wait for the newly created thread to complete
    new_thread.join().unwrap();
}
```

The thread will never end. If you are using an antique computer, please be mentally prepared before running the above code. In actual code, always be alert to this situation (in the case of a single core, the CPU usage will soar to 100%), unless you do it on purpose.

Another way for a thread to end is that the process in which the thread resides ends. Let's modify the above example slightly:

```rust
use std::thread;

fn main() {
    // create a thread
    thread::spawn(move || {
        loop {
            println!("I am a new thread.");
        }
    });

    // Don't wait for the newly created thread to complete
    // new_thread.join().unwrap();
}
```
Compared with the above code, the only difference is that the last line of code of the `main` function is commented, so that the main thread does not have to wait for the new thread to be created. The newly created thread is over. Here, you may have questions: Why must the end of the process cause the end of the newly created thread? Could it also be caused by the end of the main thread that created the new thread? What is the fact, we might as well verify:

```rust
use std::thread;

fn main() {
	// create a thread
    let new_thread = thread::spawn(move || {
    	// create another thread
    	thread::spawn(move || {
    		loop {
	            println!("I am a new thread.");
	        }
    	})
    });

    // Wait for the newly created thread to complete
    new_thread.join().unwrap();
    println!("Child thread is finish!");

    // Sleep for a while to see if the child thread created by the child thread is still running
    thread::sleep_ms(100);
}
```

This time we also created a thread in the newly created thread, so that the first newly created thread is the parent thread, and the main thread actively sleeps for a period of time after waiting for the parent thread to end. This has two purposes, one is to ensure that the entire program will not end immediately; the other is that if the sub-thread still exists, it should get an execution opportunity to check whether the sub-thread is still running. The following is the output:

```
Child thread is finished!
I am a new thread.
I am a new thread.
......
```

The results show that after the parent thread ends, the child thread created by it is still alive, which will not end because the parent thread ends. This is more in line with the laws of nature, otherwise, we will really lose our children and grandchildren, and human beings will become extinct. So the second way to cause a thread to end is to end its process. So far, we have introduced the creation and termination of threads, so we will introduce some more interesting things next. But before you do that, consider the practice questions below.

**Practice questions:**

There is a group of students' grades, we need to grade them, 90 points and above is A, 80 points and above is B, 70 points and above is C, 60 points and above is D, and below 60 points is E. Now it is required to write a program in Rust language for scoring, and the scoring is done by a new thread, and finally output the student number, grade, and scoring of each student. Student transcripts are randomly generated, the number of students is 100, the grade range is [0,100], and the student numbers start from 1 to 100.
