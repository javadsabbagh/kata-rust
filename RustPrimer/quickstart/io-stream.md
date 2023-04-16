# input and output stream
**Input output** is a way of human-computer interaction. The most common input and output are standard input and output and file input and output (of course there is also database input and output, which is not discussed in this section).

## standard input
Standard input, also known as console input, is a type of common input.

**Example 1:**

```rust
use std::io;

fn read_input() -> io::Result<()> {
     let mut input = String::new();

     try!(io::stdin().read_line(&mut input));

     println!("You typed: {}", input.trim());

     Ok(())
}

fn main() {
     read_input();
}
```

**Example 2:**

```rust
use std::io;
fn main() {
     let mut input = String::new();

     io::stdin().read_line(&mut input).expect("WTF!");

     println!("You typed: {}", input.trim());
}
```

Here is a common way of processing standard input. Both examples declare a mutable string to hold the input data.
Where they differ is in the way potential input exceptions are handled.

1. Example 1 uses the `try!` macro. This macro will return `Result<(), io::Error>` type, and `io::Result<()>` is an alias for this type. So example 1 needs to use a `read_input` function alone to receive this type, not in the `main` function, because the `main` function does not receive `io::Result<()>` as the return type.

2. Example 2 uses the `expect` method of `Result<(), io::Error>` type to receive the return type of `io::stdin().read_line`. And handle possible potential io exceptions.

## standard output
Standard output is also called console output. Common standard output macros in Rust include `print!` and `println!`. The difference between them is that the latter outputs one more newline at the end than the former.

**Example 1:**

```rust
fn main() {
     print!("this");
     print!("will ");
     print!("be ");
     print!("on ");
     print!("the ");
     print!("same");
     print!("line");

     print!("this string has a newline, why not choose println! instead?\n");
}
```

**Example 2:**

```rust
fn main() {
     println!("hello there!");
     println!("format {} arguments", "some");
}
```

Both examples here are relatively simple. Readers can run it to view the output and compare their differences.
It is worth noting that in example 2, `{ }` will be replaced by `"some"`. This is a formatted output in rust.

Normalized output is line-buffered, which means that normalized output is not implicitly flushed until a new line is encountered.
In other words `print!` and `println!` do not always have the same effect.
To put it more simply and clearly, you cannot treat `print!` as `printf` in C language. For example:

```rust
use std::io;
fn main() {
     print!("Please enter a string: ");
     let mut input = String::new();
     io::stdin()
         .read_line(&mut input)
         .expect("Read failed");
     print!("The string you entered is: {}\n", input);
}
```

When this code runs, the expected prompt string does not appear first because the row is not flushed.
If you want to achieve the expected effect, you need to refresh the display:

```rust
use std::io::{self, Write};
fn main() {
     print!("Please enter a string: ");
     io::stdout().flush().unwrap();
     let mut input = String::new();
     io::stdin()
         .read_line(&mut input)
         .expect("Read failed");
     print!("The string you entered is: {}\n", input);
}
```

## file input

File input is similar to standard input, except that the input stream points to a file instead of the console. The following example uses pattern matching to handle potential typos

**example:**

```rust
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
     // Create a file path
     let path = Path::new("hello.txt");
     let display = path.display();

     // Open the file in read-only mode, return a `io::Result<File>` type
     let mut file = match File::open(&path) {
         // Handle potential errors in opening the file
         Err(why) => panic!("couldn't open {}: {}", display,
                                                    Error::description(&why)),
         Ok(file) => file,
     };

     // File input data to string and return `io::Result<usize>` type
     let mut s = String::new();
     match file. read_to_string(&mut s) {
         Err(why) => panic!("couldn't read {}: {}", display,
                                                    Error::description(&why)),
         Ok(_) => print!("{} contains:\n{}", display, s),
     }
}
```

## file output
File output is similar to standard library output, except that the output stream is redirected to a file. See the example below for details.

**example:**

```rust
// output text
static LOREM_IPSUM: &'static str =
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercise ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidat non
proident, sunt in culpa qui officia desert mollit anim id est laborum.
";

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
     let path = Path::new("out/lorem_ipsum.txt");
     let display = path.display();

     // Open a file in write-only mode and return `io::Result<File>` type
     let mut file = match File::create(&path) {
         Err(why) => panic!("couldn't create {}: {}",
                            display,
                            Error::description(&why)),
         Ok(file) => file,
     };

     // Write `LOREM_IPSUM` string to the file, and return `io::Result<()>` type
     match file.write_all(LOREM_IPSUM.as_bytes()) {
         Err(why) => {
             panic!("couldn't write to {}: {}", display,
                                                Error::description(&why))
         },
         Ok(_) => println!("successfully wrote to {}", display),
     }
}
```
