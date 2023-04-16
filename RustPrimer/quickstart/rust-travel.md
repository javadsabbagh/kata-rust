#Rustjourney

## Hello World
According to the tradition of programming languages, the first program to learn the first programming language is to print Hello World!
Let's follow our steps to create Rust's Hello World! program:

**The following command operations, if not specified, are all run under the shell. For the sake of simplicity and unity in this article, all examples are run under win10 powershell, and all commands are run after the `ps:` identifier**

- Create a Doing directory and helloworld.rs file

> ps: mkdir ~/Doing
> ps: cd ~/Doing
> ps: notepad helloworld.rs # The author prefers to use sublime as the editor
> ps: subl helloworld.rs # After this chapter, use subl instead of notepad

Note that the suffix used here is .rs. Code files in general programming languages have customary suffixes, such as:
     C language is .c, java is .java, python is .py, etc. ** Please remember that the customary suffix of Rust language is .rs** (although other suffixes can also be compiled by rustc).

- Enter the Rust code in the helloworld.rs file

```rust
fn main() {
     println!("Hello World!");
}
```

- Compile the helloworld.rs file

> ps: rustc helloworld.rs
> ps: rustc helloworld.rs -O # You can also choose to compile with optimization

- Run the program

> ps: ./helloworld.exe # Windows platform needs to add .exe suffix
> Hello World!

Without `ps:` prefix means console printout.

We've written our first executable in rust, and it prints 'hello world!', cool, right!
But what does this code mean? As a novice, you must be confused. Let's take a look at this program first:

1. In the first line, fn means to define a **function**, main is the name of this function, and the statement in curly braces {} indicates the content of this function.
2. The function named **main** has a special purpose, that is, as the entry point of the program, that is to say, the program starts to run from this function every time.
3. There is only one sentence in the function ```println!("Hello World!");```, where ```println!``` is a **macro** that comes with the Rust language,
The function of this macro is to print text (with a newline at the end), and "Hello World!", which is wrapped in quotation marks, is a **string**, which is the text we want to print.
4. You must have noticed ```;```, in the Rust language, the semicolon ```;``` is used to separate the statement, that is to say, the end of the statement is generally ended with a semicolon sign.

## Hello Rust

- create project hellorust

> ps: cargo new hellorust --bin

- View directory structure

> ps: tree # win10 powershell comes with the function of tree to view the file directory structure
> └─ hellorust
> ----└─src

The directory structure shown here has a src folder and a Cargo.toml file in the hellorust directory, and this directory will be initialized as a git project

- View the Cargo.toml file

> ps: cat Cargo.toml  
> [package]  
name = "hellorust"  
version = "0.1."  
authors = ["YourName <YourEmail>"]  
[dependencies]  

- Edit the main.rs file in the src directory

> ps: subl ./src/main.rs

The project created by cargo will have an initialized main.rs file in the src directory, the content is:

```rust
fn main() {
     println!("Hello, world!");
}
```

Now we edit this file and change it to:

```rust
fn main() {
     let rust = "Rust";
     println!("Hello, {}!", rust);
}
```

Here `let rust = "Rust"` is to bind the rust variable to "Rust" ,
In `println!("Hello, {}!", rust);`, substitute the value of the rust variable into `{}` in `"Hello, {}!"`.

- compile and run

> ps: cargo build
> ps: cargo build --release # This belongs to optimized compilation
> ps: ./target/debug/hellorust.exe
> ps: ./target/release/hellorust.exe # If the previous compilation is optimized, then run like this
> ps: cargo run # compile and run together
> ps: cargo run --release # Same as above, the difference is that it is compiled with optimization
