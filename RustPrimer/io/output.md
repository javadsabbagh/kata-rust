# print! macro

We mentioned line buffering of standard output in the quickstart. One of its manifestations is the `print!` macro. If you put an input after the `print!` macro, you will find this line-buffering mechanism.

```rust
fn main() {
	print!("hello!\ninput:");
	let mut input = String::new();
		std::io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
	println!("line:{}",input);
}
```

You can try compiling and running this program, you will find that we did not get what we expected (the underscore represents the cursor position):

```
hello!
input:_
```

Instead got:

```
hello!
_
```

This is because of the line buffering mechanism in the standard output, the content of the output will not be refreshed implicitly before a newline character is encountered, which leads to the fact that the `print!` macro and the `println!` macro are not exactly the same . The `print!` macro in the standard library looks like this:

```rust
macro_rules! print {
    ($($arg:tt)*) => { ... };
}
```

From this, we can improve it so that it and the `println!` macro are flushed automatically, but this flush is an explicit flush.

```rust
use std::io::{self, Write};

macro_rules! printf {
	($($arg:tt)*) =>{
		print!($($arg)*);
		io::stdout().flush().unwrap();
	}
}
```

In addition, when you need to refresh a line that has not encountered a line break, you can use `io::stdout().flush().unwrap();` to refresh, but it should be noted that `use std::io::{self, Write};` If you don't do this, you will get an error.
