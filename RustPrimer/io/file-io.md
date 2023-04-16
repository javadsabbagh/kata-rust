# file input and output

The file `std::fs::File` itself implements the `Read` and `Write` traits, so the input and output of the file is very simple, as long as you get an instance of the `File` type, you can call the read and write interface to perform file input and output operations up. To get `File`, you have to let the operating system open (open) or create a new file (create). Or take an example to illustrate

```rust
use std::io;
use std::io::prelude::*;
use std::fs::File;

// create file and write something
fn create_file(filename: &str, buf: &[u8]) -> io::Result<()> {
	let mut f = try!(File::create(filename));
	try!(f.write(&buf));
	Ok(())
}

// read from file to String
fn read_file(filename: &str, buf: &mut String) -> io::Result<()> {
	let mut f = try!(File::open(filename));
	try!(f.read_to_string(&buf));
	Ok(())
}

fn main() {
	let f = "foo.txt";
	let mut buf = String::new();
	match create_file(f, b"Hello, World!") {
		Ok(()) => {
		    match read_file(f, &mut buf) {
		        Ok(()) => {println!("{}", buf);},
		        Err(e) => {println!("{}", e);},
            };
		},
		Err(e) => {println!("{}", e);},
	}
}
```

Rust’s handling of file operations is somewhat different from other languages. Other languages generally pass read and write options as function parameters to the open function, while Rust calls the open function on the option. [`std::fs::OpenOptions`](http://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html) is a builder, which can be chained after being created by the new function Set the option to open the file, whether it is read, write, append, truncate or create, etc. After the OpenOptions is built, you can then call the open method. See the following example to understand

```rust
use std::fs::OpenOptions;

let file = OpenOptions::new().write(true).truncate(true).open("foo.txt");
```

Rust uses the builder pattern to set open file options. Compared with passing options to the open function with characters as parameters, one advantage is that it allows the compiler to ensure that the options are checked for validity, and you don’t have to wait until runtime to find out that the read- The `r` of mode is written as `t`.
