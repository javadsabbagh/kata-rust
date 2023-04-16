# standard input and output

Recall that the first Rust program we wrote had side effects, and its side effect was to output to the standard output (stdout), usually a terminal or a screen, and output Hello, World! to light up the place where these characters are on the screen. The `println!` macro is the most common output, and `print!` is also used to output macros. Both of them output to the standard output (stdout), and the difference between the two can be seen at a glance. As for formatted output, [Basic Operators and String Formatting Section](../type/operator-and-formatting.md) has detailed descriptions, so I won’t repeat it here.

The more general standard input and output are defined in the `std::io` module, calling the `std::io::stdin()` and `std::io::stdout()` two functions will get the input handle respectively And the output handle (hey, the word [handle](https://zh.wikipedia.org/wiki/%E5%8F%A5%E6%9F%84) is the most inexplicable translation in the history of computers), these two By default, the handle will be synchronized through a mutex, which means that multiple processes are not allowed to read or write standard input and output at the same time. Otherwise, if one process wants to draw a horse to the standard output, one process wants to draw a donkey, and two processes write to the standard output at the same time If so, a mule may be drawn in the end. If there are more processes to draw different animals, it may end up being completely different. In addition to implicitly using a mutex, we can also explicitly call `.lock()` on the handle. The input and output handle implements the read and write traits mentioned above, so it is a reader/writer, and the interface can be adjusted to read and write standard input and output. Give a few chestnuts:

```rust
use std::io;

fn read_from_stdin(buf: &mut String) -> io::Result<()> {
try!(io::stdin().read_line(buf));
Ok(())
}
```

```rust
use std::io;

fn write_to_stdout(buf: &[u8]) -> io::Result<()> {
try!(io::stdout().write(&buf));
Ok(())
}
```

It can be seen that the above examples all return the `io::Result<()>` type. This is not accidental, but a common way of writing IO operations, because IO operations are programs that deal with the outside world, so they are all likely to fail. , use `io::Result<T>` to wrap the result, `io::Result<T>` is just the type of `E` fixed to `io::Error` in the standard `Result<T,E>` Alias, and as an operation with side effects, we generally don’t need to care about its return value, because the real meaning of executing such functions is reflected in the side effects, so the return value is only used to indicate whether the execution is successful, and `Result` itself The type itself can already represent the execution state, and it doesn’t matter what `T` is in it. Since `T` is meaningless, then we can choose the meaningless `unit` type, so IO operations basically use` io::Result<()>`.

Another thing to note is that because the IO operation may fail, it is generally used together with the `try!` macro, but `try!` will return the error when it encounters an error, so you need to ensure The return type of the function containing the `try!` statement is `io::Result<T>`. Many beginners directly check the std api documentation without carefully reading the documentation, and then follow the examples in the api documentation to use the `try with IO operation The !` macro is written in the `main` function. As a result, after compiling, wiping, and writing according to the document, it can't be compiled. What a bad document. In fact, click the run button on the api document, and you will find that the examples in the document all put `try!` in another function, because the `main` function has no return value, and `try!` will return` io::Result<T>`, so putting `try!` directly in the `main` function must be kneeling. For example, the following reads a line of input from standard input, because `try!` is placed in the main function, it cannot be compiled.

```rust
use std::io;

fn main() {
let mut input = String::new();
try!(io::stdin().read_line(&mut input));
println!("You typed: {}", input.trim());
}
```

One important thing to note here is that there is no way in Rust to get a numeric value from the keyboard. In fact, a language like C does not directly obtain the number type, it just does a conversion. So what should we do if we want to get a number type from the keyboard?

```rust
fn main() {
let mut input = String::new();
std::io::stdin()
.read_line(&mut input)
.expect("Failed to read line");
     // The equivalent writing here is:
     // let num: i32 = input.trim().parse().unwrap();
let num = input.trim().parse::<i32>().unwrap();
println!("The number you entered is: {}", num);
}
```

If you need to enter numbers in many places, you can write a `numin` macro yourself:

```rust
macro_rules! numin {
() => {
{
             let mut input = String::new();
std::io::stdin()
.read_line(&mut input)
                 .expect("Failed to read line");
input.trim().parse().unwrap()
         }
     };
}
```

So the above program can be rewritten as:

```

fn main() {
     let num: i32 = numin!();
println!("The number you entered is: {}", num);
}
```

However, if the user enters something other than a number, an error will result. This is very similar to C. Of course, you can make the program more complicated to ensure that the user must input numbers. But these are not what we will discuss in this section.

Another point that some programmers transferred from other languages may wonder is how to accept input parameters from the command line, because the main function in C can take parameters, so the input parameters can be obtained directly from the parameters of the main function. But in fact, this type of input is very different from what we are talking about here. It is classified as an environment variable in Rust and can be obtained through `std::env::args()`. This function returns an `Args` iteration device, the first of which is the program name, and the latter are the command line parameters input to the program.

```rust
use std::env;

fn main() {
	let args = env::args();
	for arg in args {
		println!("{}", arg);
	}
}
```

Save the above program as *args.rs* and compile and execute, the result is as follows

```
$ rustc args.rs
$ ./args a b c
./args
a
b
c
```
