# Directory operation: simple grep

In the previous section, we realized calling subprocess through `Command`. In this section, we will implement a simple grep through our own code. Of course, you can find the source code of this basic tool, and our implementation does not focus on efficiency like the real grep. The main function of this section is to demonstrate the use of the standard library API.

First, we need to recurse and traverse the current directory, and whenever we find a file, we call back a function.

So, we have this function:

```rust
use std::env::args;
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;

fn visit_dirs(dir: &Path, pattern: &String, cb: &Fn(&DirEntry, &String)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), pattern, cb));
            } else {
                cb(&entry, pattern);
            }
        }
    }else{
        let entry = try!(try!(fs::read_dir(dir)).next().unwrap());
        cb(&entry, pattern);
    }
    Ok(())
}

```

We have such a function, some students may find this code familiar. Isn't this the example in the standard library changed?

.

.

.

yes!

Well, continue, we need to read each found file, and at the same time judge whether there is what we are looking for in each line.
We use a BufferIO to read each file, and use String's own method to determine whether the content exists.

```rust
fn call_back(de: &DirEntry, pt: &String) {
    let mut f = File::open(de.path()).unwrap();
    let mut buf = io::BufReader::new(f);
    for line in io::BufRead::lines(buf) {
        let line = line.unwrap_or("".to_string());
        if line.contains(pt) {
            println!("{}", &line);
        }
    }
}
```

Finally, we call the entire function as follows:

```rust
use std::env::args;
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;

fn visit_dirs(dir: &Path, pattern: &String, cb: &Fn(&DirEntry, &String)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), pattern, cb));
            } else {
                cb(&entry, pattern);
            }
        }
    }else{
        let entry = try!(try!(fs::read_dir(dir)).next().unwrap());
        cb(&entry, pattern);
    }
    Ok(())
}

fn call_back(de: &DirEntry, pt: &String) {
    let mut f = File::open(de.path()).unwrap();
    let mut buf = io::BufReader::new(f);
    for line in io::BufRead::lines(buf) {
        let line = line.unwrap_or("".to_string());
        if line.contains(pt) {
            println!("{}", &line);
        }
    }
}

// Realize calling the grep command to search for files
fn main() {
    let mut arg_iter = args();
    arg_iter.next();
    // panic if there is no one
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt =  arg_iter.next().unwrap_or("./".to_string());
    let pt = Path::new(&pt);
    visit_dirs(&pt, &pattern, &call_back).unwrap();
}

```

The call is as follows:

```
➜ demo git:(master) ✗ ./target/debug/demo "fn main()" ../
fn main() {
fn main() { }
fn main() {
    pub fn main() {
    pub fn main() {}
fn main() {
    pub fn main() {
    pub fn main() {}
```
