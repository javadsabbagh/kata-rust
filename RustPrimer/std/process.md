# System command: call grep

We know that there is a command called grep in the Linux system, which can analyze the target file and find the corresponding string, and output the line where the string is located.
Today, let's write a Rust program to call this grep command

```rust
use std::process::*;
use std::env::args;

// Realize calling the grep command to search for files
fn main() {
    let mut arg_iter = args();
    // panic if there is no one
    arg_iter.next().unwrap();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt =  arg_iter.next().unwrap_or("./".to_string());
    let output = Command::new("/usr/bin/grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        .output()
        .unwrap_or_else(|e| panic!("wg panic because:{}", e));
    println!("output:");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");
    for line in lines {
        println!("{}", line);
    }
}

```

It seems to be good, but the above program has a fatal shortcoming - because Output is synchronous, so once there are huge files in the calling directory, grep analysis will take a huge amount of time. This is not allowed for a highly available program.

So how to improve?

In fact, in the above code, we hide a `Child` concept, that is - child process.

Let me demonstrate how to operate the subprocess:

```rust
use std::process::*;
use std::env::args;

// Realize calling the grep command to search for files
fn main() {
     let mut arg_iter = args();
     // panic if there is no one
     arg_iter.next();
     let pattern = arg_iter.next().unwrap_or("main".to_string());
     let pt = arg_iter.next().unwrap_or("./".to_string());
     let child = Command::new("grep")
         .arg("-n")
         .arg("-r")
         .arg(&pattern)
         .arg(&pt)
         .spawn().unwrap();
     // do something else
     std::thread::sleep_ms(1000);
     println!("{}", "The calculation is time-consuming...");
     let out = child.wait_with_output().unwrap();
     let out_str = String::from_utf8_lossy(&out.stdout);
     for line in out_str. split("\n") {
         println!("{}", line);
     }
}

```

However, this example is not quite what we expected!

```
./demo main /home/wayslog/rust/demo/src
/home/wayslog/rust/demo/src/main.rs:5:fn main() {
/home/wayslog/rust/demo/src/main.rs:9: let pattern = arg_iter.next().unwrap_or("main".to_string());
Calculations are time consuming...

```

why?

Very simple, we know that in Linux, the function from `fork` will inherit all the handles of the parent process. Therefore, the child process will also inherit the standard output of the parent process, which causes such a problem. This is also the last thing we know that we can't receive the final output with out, because it has been output before!

So how to do it? Just give this child process a pipeline!

```rust
use std::process::*;
use std::env::args;

// Realize calling the grep command to search for files
fn main() {
     let mut arg_iter = args();
     // panic if there is no one
     arg_iter.next();
     let pattern = arg_iter.next().unwrap_or("main".to_string());
     let pt = arg_iter.next().unwrap_or("./".to_string());
     let child = Command::new("grep")
         .arg("-n")
         .arg("-r")
         .arg(&pattern)
         .arg(&pt)
         // set the pipeline
         .stdout(Stdio::piped())
         .spawn().unwrap();
     // do something else
     std::thread::sleep_ms(1000);
     println!("{}", "The calculation is time-consuming...");
     let out = child.wait_with_output().unwrap();
     let out_str = String::from_utf8_lossy(&out.stdout);
     for line in out_str.split("\n") {
         println!("{}", line);
     }
}
```

This code is equivalent to giving `stdout` a buffer, which is not read until after our calculation is completed, so it will not cause the problem of out-of-order output.

One thing to note here is that once you start a child process, no matter how your program handles it, you must remember to call `wait` or `wait_with_output` on this `child`, unless you explicitly call `kill`. Because if the parent process doesn't `wait` it, it will become a zombie process! ! !

*Note*: The above problem is a daily problem of Python multi-process under Linux, which is no longer surprising.
