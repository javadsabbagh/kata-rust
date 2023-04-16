# 17. Error handling
Error handling is the premise to ensure the robustness of the program. There are roughly two ways of error handling in programming languages: throwing exceptions (exceptions) and returning as a value.

**Rust** returns errors as values and provides native elegant error handling.

Mastering error handling is a very important part of software engineering. Let me take a look at the art of error handling that **Rust** shows us.

## 17.1 Option and Result
Use `panic` with caution:

```rust
fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
    }
    n == 5
}

fn main() {
    guess(11);
}
```

`panic` will cause the end of the current thread, or even the end of the entire program, which is often an undesirable result. (`panic` is a good suggestion when writing examples or short code)


### Option

```rust
enum Option<T> {
    None,
    Some(T),
}
```

**Option** is Rust's system type, which is used to represent the possibility that the value does not exist. This is a good practice in programming, and it forces **Rust** to detect and handle the situation where the value does not exist. For example:

```rust
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}
```

`find` looks for `needle` characters in the string `haystack`. In fact, there are two possibilities for the result, yes (`Some(usize)`) or no (`None`).

```rust
fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i+1..]),
    }
}
```

**Rust** uses pattern matching to handle return values, and the caller must handle the case where the result is `None`. This is often a good programming practice and reduces potential bugs. **Option** contains some methods to simplify pattern matching. After all, too many `match` will make the code bloated, which is one of the reasons for bugs.

#### unwrap

```rust
impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None =>
              panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

`unwrap` will panic when it encounters `None` value, as mentioned earlier, this is not a good engineering practice. But sometimes it is very useful:

* **In examples and simple and fast coding** Sometimes you just need a small example or a simple small program, the input and output have been determined, you don't need to spend too much time thinking about error handling, use `unwrap` become very suitable.
* **When the program encounters a fatal bug, panic is the best choice**


#### map

If we want to find the file extension in a string, such as `rs` in `foo.rs`, we can do this:

```rust
fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
    }
}

fn main() {
    match extension_explicit("foo.rs") {
        None => println!("no extension"),
        Some(ext) =>  assert_eq!(ext, "rs"),
    }
}
```

We can simplify this using `map`:

```rust
// map is a method in the standard library
fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}
// Use map to remove match
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}
```

If `map` has value `Some(T)`, `f` will be executed, otherwise `None` will be returned directly.

#### unwrap_or

```rust
fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        None => default,
        Some(value) => value,
    }
}
```
`unwrap_or` provides a default value `default`, and returns `default` when the value is `None`:
```rust
fn main() {
    assert_eq!(extension("foo.rs").unwrap_or("rs"), "rs");
    assert_eq!(extension("foo").unwrap_or("rs"), "rs");
}
```

#### and_then

```rust
fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
        where F: FnOnce(T) -> Option<A> {
    match option {
        None => None,
        Some(value) => f(value),
    }
}
```

It seems that `and_then` is similar to `map`, but `map` just remaps the value of `Some(t)`, and `and_then` will return another `Option`. This is especially important if we find its extension in a file path:

```rust
use std::path::Path;
fn file_name(file_path: &str) -> Option<&str> {
    let path = Path::new(file_path);
    path.file_name().to_str()
}
fn file_path_ext(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension)
}
```

### Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` is a more general version of `Option`, which explains why the result is wrong compared to `Option` which results in `None`, so:

```rust
type Option<T> = Result<T, ()>;
```

Such aliases are the same (`()` denotes the empty tuple, which is both `()` type and `()` value)
#### unwrap

```rust
impl<T, E: ::std::fmt::Debug> Result<T, E> {
    fn unwrap(self) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(err) =>
              panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
        }
    }
}
```

That's right, they are the same as `Option`, in fact they have many similar methods, the difference is that `Result` includes a detailed description of the error, which is friendly to debuggers.

#### Result Let's start with the example

```rust
fn double_number(number_str: &str) -> i32 {
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let n: i32 = double_number("10");
    assert_eq!(n, 20);
}
```

`double_number` parses an `i32` number from a string and `*2`, calling in `main` seems to be no problem, but if `"10"` is replaced by other string programs that cannot be parsed will panic

```rust
impl str {
    fn parse<F: FromStr>(&self) -> Result<F, F::Err>;
}

```

`parse` returns a `Result`, but let's also return an `Option`, after all a string can either be parsed into a number or it can't, but a `Result` gives us more information (either an empty string, an invalid number, too large or too small), which is user-friendly. When you are faced with a choice between Option and Result. If you can provide detailed error messages, then presumably you should too.

Here you need to understand the **trait** of `FromStr`:

```rust
pub trait FromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

impl FromStr for i32 {
    type Err = ParseIntError;
    fn from_str(src: &str) -> Result<i32, ParseIntError> {

    }
}
```

`number_str.parse::<i32>()` actually calls the `FromStr` implementation of `i32`.

We need to rewrite this example:

```rust
use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn main() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }
}
```

Not only `map`, `Result` also includes `unwrap_or` and `and_then`. There are also methods `map_err` and `or_else` specific to error types.

#### Result Alias
The alias of Result often appears in **Rust** standard library, which is used to confirm the type of `Ok(T)` or `Err(E)` by default, which can reduce repeated coding. For example `io::Result`

```rust
use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, ParseIntError>;

fn double_number(number_str: &str) -> Result<i32> {
    unimplemented!();
}
```

### Combine Option and Result
`Option` method `ok_or`:

```rust
fn ok_or<T, E>(option: Option<T>, err: E) -> Result<T, E> {
    match option {
        Some(val) => Ok(val),
        None => Err(err),
    }
}
```

Can return a `Result::Err(E)` when the value is `None`, and return `Ok(T)` when the value is `Some(T)`, using it we can combine `Option` and `Result `:

```rust
use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

`double_arg` converts the incoming command line parameters into numbers and doubles them, `ok_or` converts the `Option` type into `Result`, and `map_err` calls the function processing as a parameter when the value is `Err(E)` mistake

#### Complicated example

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
         .map_err(|err| err.to_string())
         .and_then(|mut file| {
              let mut contents = String::new();
              file.read_to_string(&mut contents)
                  .map_err(|err| err.to_string())
                  .map(|_| contents)
         })
         .and_then(|contents| {
              contents.trim().parse::<i32>()
                      .map_err(|err| err.to_string())
         })
         .map(|n| 2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

`file_double` reads content from a file and converts it to an `i32` type and doubles it.
This example already looks very complicated, it uses multiple composition methods, we can use the traditional `match` and `if let` to rewrite it:

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

These two methods are both acceptable in personal opinion, depending on the specific situation.

### try!

```rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}

```

`try!` is actually the encapsulation of `match Result`, it will return early when encountering `Err(E)`,
`::std::convert::From::from(err)` can return different error types into the final required error type, because all errors can be converted into `Box<Error>` through `From`, So the following code is correct:

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n = try!(contents.trim().parse::<i32>());
    Ok(2 * n)
}

```

#### Combining custom error types

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::io;
use std::path::Path;

// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `CliError` values.
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn file_double_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path).map_err(CliError::Io));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(CliError::Io));
    let n: i32 = try!(contents.trim().parse().map_err(CliError::Parse));
    Ok(2 * n)
}
```

`CliError` implements the `From` trait for `io::Error` and `num::ParseIntError` respectively, and these two error types can be converted to `CliError` when calling `try!`.

### Summarize

Proficiency in using `Option` and `Result` is the key to writing **Rust** codes. **Rust** elegant error handling is inseparable from the error form of value return. The detailed error information provided to users when writing code is worthy of praise.
