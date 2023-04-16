## Message Passing
After a little consideration, the exercises in the previous section are actually incomplete. They are just a part of the scoring system. A scoring system needs to read information from the database or files first, then score, and finally need to Save the scoring results to the database or file. If you do these three steps serially step by step, there is no problem at all. So can we use three threads to do these three steps separately? In the last practice question, we have used one thread to achieve scoring, so can we also use another thread to read the scores, and then use another thread to achieve saving? If this is the case, then we can take advantage of multi-core and multi-cpu to speed up the efficiency of the entire scoring. Now that the question is asked here, the answer is obvious. The question is how do we implement it in Rust, the key is how do the three threads exchange information to achieve a serial logical processing sequence?

To address this, the following describes a messaging technique that Rust supports in the standard library. **Message passing** is a model that everyone respects in the concurrency model, not only because it is relatively simple to use, but the key is that it can reduce data competition and improve concurrency efficiency, so it is worth studying in depth. Rust implements this pattern through something called a channel (`channel`), let's go directly to the topic.

### Initial test channel (channel)
Rust's channel (`channel`) can pass messages (data) from one thread to another thread, allowing information to flow in different threads to achieve collaboration. See [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html) for details. The two ends of the channel are the sender (`Sender`) and the receiver (`Receiver`). The sender is responsible for sending a message from one thread, and the receiver receives the message in another thread. Let's look at a simple example:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // create a channel
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = 
        mpsc::channel();

    // Create a thread for sending messages
    thread::spawn(move || {
        // Send a message, here is the numeric id
        tx.send(1).unwrap();
    });

    // Receive the message sent by the child thread in the main thread and output it
    println!("receive {}", rx.recv().unwrap());
}
```

For the program description, see the comments in the code. The result of the program execution is:

```
receive 1
```

The result shows that the main thread where `main` is located has received the message sent by the newly created thread. It is so simple to pass messages between threads with Rust!

Although it is simple, if you have used other languages, you will know that there are many ways to use the channel, and it is relatively flexible. For this reason, we need to further consider several issues about the `Channel` of `Rust`:

1. Can the channel guarantee the order of messages? Is the message sent first, received first?
2. Can the channel cache messages? If so how much can be cached?
3. Do the sender and receiver of the channel support N:1, 1:N, N:M modes?
4. Can the channel send any data?
5. Is there any problem in continuing to use the sent data in the thread?

Let's take these questions and reflections into the next section, where there are relevant answers.

### Message Type
In the above example, the message type we pass is `i32`, besides this type, is it possible to pass more primitive types, or more complex types, and custom types? Below we try to send a more complex `Rc` type message:

```rust
use std::fmt;
use std::sync::mpsc;
use std::thread;
use std::rc::Rc;

pub struct Student {
    id: u32
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "student {}", self.id)
    }
}

fn main() {
    // create a channel
    let (tx, rx): (mpsc::Sender<Rc<Student>>, mpsc::Receiver<Rc<Student>>) =
        mpsc::channel();

    // Create a thread for sending messages
    thread::spawn(move || {
        // Send a message, here is the numeric id
        tx.send(Rc::new(Student{
            id: 1,
        })).unwrap();
    });

    // Receive the message sent by the child thread in the main thread and output it
    println!("receive {}", rx.recv().unwrap());
}
```

Compile the code, the miracle does not appear, there is a compile-time error, and the error message:

```
error: the trait `core::marker::Send` is not
implemented for the type `alloc::rc::Rc<Student>` [E0277]
note: `alloc::rc::Rc<Student>` cannot be sent between threads safely
```

It seems that not all types of messages can be sent through channels, the message type must implement `marker trait Send`. The reason why Rust makes such a mandatory requirement is mainly to solve the problem of concurrency safety. Once again, **safety** is the top priority of Rust. If a type is `Send`, it indicates that it can safely transfer ownership (`ownership`) between threads. When ownership is transferred from one thread to another, only one thread can access it at a time, so It avoids data races and thus achieves thread safety. The power of `ownership` is shown once again. Through this approach, all codes must meet this contract at compile time. This method is worth learning, and `trait` is also very powerful.

It seems that the problem has been perfectly solved. However, since `Send` itself is an unsafe `marker trait`, and there is no actual `API`, it is very simple to implement, but there is no mandatory guarantee, and it can only be done by developers. Restrain yourself, otherwise it may still cause concurrent security issues. Don't worry too much about this, because the classes that already exist in Rust have already implemented `Send` or `!Send`, we just need to use it. `Send` is a trait that applies to all existing Rust classes by default, so we use `!Send` to explicitly indicate that the class does not implement `Send`. Currently almost all primitive types are `Send`, such as `i32` sent in the previous example. For developers, we may be more concerned about whether `Send` is implemented, that is, `!Send` is implemented, because this will lead to thread insecurity. For more comprehensive information, see [`Send` official website API](https://doc.rust-lang.org/std/marker/trait.Send.html).

For the case of not `Send` (`!Send`), it can be roughly divided into two categories:

1. Raw pointers, including `*mut T` and `*const T`, because different threads can access data through pointers, which may cause thread safety issues.
2. `Rc` and `Weak` are not, because the reference count will be shared, but there is no concurrency control.

Although there are these cases of `!Send`, they cannot escape the eyes of the compiler. As long as you use the message type incorrectly, the compiler will give an error message similar to the above. This is not what we have to worry about, because errors are more likely to appear in newly created custom classes. There are two points to note:

1. If all fields of a custom class are `Send`, then this custom class is also `Send`.
     Conversely, if there is a field that is `!Send`, then this custom class is also `!Send`.
     If the field of the class has recursive inclusion, follow this principle to deduce whether the class is `Send` or `!Send`.

2. When implementing `Send` or `!Send` for a custom class, you must ensure that it conforms to its contract.

At this point, the relevant knowledge of message types has been introduced. After talking for so long, it is time for everyone to practice: please implement a custom class, which contains an Rc field, so that this class can be sent in the channel message type.

### Asynchronous Channel (Channel)
After tentatively experimenting with channels, it's time to dig a little deeper. Rust's standard library actually provides two types of channels: asynchronous channels and synchronous channels. The above examples all use asynchronous channels. For this reason, we give priority to further introducing asynchronous channels in this section, and then introduce synchronous channels later. Asynchronous channel refers to: regardless of whether the receiver is receiving the message, the message sender will not block when sending the message. To verify this, we try to add one more thread to send messages:

```rust
use std::sync::mpsc;
use std::thread;

// number of threads
const THREAD_COUNT :i32 = 2;

fn main() {
    // create a channel
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // Create a thread for sending messages
    for id in 0..THREAD_COUNT {
        // Note that Sender can be cloned, so that multiple senders can be supported
        let thread_tx = tx.clone();
        thread::spawn(move || {
            // Send a message, here is the numeric id
            thread_tx.send(id + 1).unwrap();
            println!("send {}", id + 1);
        });
    }

    thread::sleep_ms(2000);
    println!("wake up");
    // Receive the message sent by the child thread in the main thread and output it
    for _ in 0..THREAD_COUNT {
        println!("receive {}", rx.recv().unwrap());
    }
}
```

operation result:

```
send 1
send 2
wake up
receive 1
receive 2
```

In the code, we deliberately let the main thread where `main` is located sleep for 2 seconds, so that the thread where the sender is located is executed first. From the results, we can find that the sender does not block when sending messages. Remember how I mentioned a lot about channels earlier? Did you find anything else from this example? In addition to non-blocking, we can also find three other characteristics:

1. The channel can support multiple senders at the same time, which is realized by `clone`.
     This is similar to the sharing mechanism of `Rc`.
     In fact, you can also know this from the library name `std::sync::mpsc` where `Channel` is located.
     Because `mpsc` is short for Multiple Producers Single Consumer.
     There can be multiple senders, but only one receiver, which is the supported N:1 mode.

2. The asynchronous channel has the function of message caching, because 1 and 2 are sent before they are received, and these two messages can still be received after that.

So how many messages can the channel cache? In theory it is infinite, just try it out:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // create a channel
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // Create a thread for sending messages
    let new_thread = thread::spawn(move || {
        // Send infinitely many messages
        let mut i = 0;
        loop {
            i = i + 1;
            // add code here
            println!("send {}", i);
            match tx.send(i) {
                Ok(_) => (),
                Err(e) => {
                    println!("send error: {}, count: {}", e, i);
                    return;
                },
            }
        }
    });

    // Receive the message sent by the child thread in the main thread and output it
    new_thread.join().unwrap();
    println!("receive {}", rx.recv().unwrap());
}
```

The end result is memory consumption.

3. The order of message sending and receiving is consistent, satisfying the first-in-first-out principle.

Most of the content introduced above is about the sender and the channel. Let's start to examine the receiving end. Through the above examples, you may have noticed that the receiver's `recv` method should block the current thread. If it is not blocked, in the case of multi-threading, it is impossible to receive all the sent messages. So there is no sender to send a message, so the receiver will wait forever, which is something to keep in mind. In some scenarios, waiting all the time is in line with actual needs. But in some cases, you don’t need to wait all the time, then you can consider releasing the channel. As long as the channel is released, the `recv` method will return immediately.

The asynchronous channel has good flexibility and scalability. It can be flexibly applied to actual projects according to business needs. It is a must-have medicine!

### Synchronization channel
The use of synchronous channels is the same as that of asynchronous channels, and the receiving end is the same. The only difference is the sending end. Let's look at the following example first:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a synchronous channel
    let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(0);

    // Create a thread for sending messages
    let new_thread = thread::spawn(move || {
        // Send a message, here is the numeric id
        println!("before send");
        tx.send(1).unwrap();
        println!("after send");
    });

    println!("before sleep");
    thread::sleep_ms(5000);
    println!("after sleep");
    // Receive the message sent by the child thread in the main thread and output it
    println!("receive {}", rx.recv().unwrap());
    new_thread.join().unwrap();
}
```

operation result:

```
before sleep
before send
after sleep
receive 1
after send
```

Except for some more output code, the above code is almost the same as the previous asynchronous channel. The only difference is the line of code that creates the synchronous channel. The sync channel is `sync_channel`, and the corresponding sender becomes `SyncSender`. In order to show the difference of the synchronous channel, some printing was added on purpose. Compared with asynchronous channels, there are two differences:

1. The synchronization channel needs to specify the number of cached messages, but it should be noted that the minimum can be 0, which means there is no cache.
2. The sender will be blocked. When the channel's buffer queue can no longer buffer the message, the sender will be blocked when sending the message.

Analyze the above two points and the running results. Since the main thread sleeps before receiving the message, the sub-thread will be scheduled to send the message at this time. Since the message that the channel can cache is 0, the receiver has not received it at this time. , so `tx.send(1).unwrap()` will block the child thread until the main thread receives the message, that is, execute `println!("receive {}", rx.recv().unwrap());` . The running results confirm this point. If there is no blockage, then `after send` should be after `before send`.

In comparison, the asynchronous channel has no sense of responsibility, because the sender of the message just sends it, regardless of whether the receiver can process it quickly. In this way, a large number of messages cached in the channel may not be processed, thus occupying a large amount of memory, and eventually leading to memory exhaustion. The synchronous channel can avoid this problem, and transmit the receiver's pressure energy to the sender, so as to pass it on all the time.
