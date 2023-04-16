# function parameters
## parameter declaration
   Rust's function parameter declaration is similar to a general variable declaration. It also adds a colon after the parameter name, and the colon is followed by the parameter type, but the `let` keyword is not required. It should be noted that ordinary variable declarations (let statements) can omit variable types, but function parameter declarations cannot omit parameter types.
   Let's look at a simple example:
  
  ```rust
fn main() {
  say_hi("ruster");
}

fn say_hi(name: &str) {
  println!("Hi, {}", name);
}
  ```
  
   In the above example, the `say_hi` function has one parameter named `name` of type `&str`.

## Pass function as parameter
   In rust, functions are first-class citizens (can be stored in variables/data structures, can be passed as parameters to functions, and can be used as return values), so function parameters of rust can not only be general types, but also functions. like:
  
  ```rust
fn main() {
  let xm = "xiaoming";
  let xh = "xiaohong";
  say_what(xm, hi);
  say_what(xh, hello);
}

fn hi(name: &str) {
  println!("Hi, {}.", name);
}

fn hello(name: &str) {
  println!("Hello, {}.", name);
}

fn say_what(name: &str, func: fn(&str)) {
  func(name)
}
  ```
  
   In the above example, both `hi` function and `hello` function have only one `&str` type parameter and no return value. The `say_what` function has two parameters, one is `&str` type, and the other is a function type (function type), which is a function type with only one `&str` type parameter and no return value.

## pattern matching
   Supporting pattern matching adds a lot of flexibility to rust, and it is very comfortable to use. Pattern matching can be used not only in variable declarations (let statements), but also in function parameter declarations, such as:
  
  ```rust
fn main() {
  let xm = ("xiaoming", 54);
  let xh = ("xiaohong", 66);
  print_id(xm);
  print_id(xh);
  print_name(xm);
  print_age(xh);
  print_name(xm);
  print_age(xh);
}

fn print_id((name, age): (&str, i32)) {
  println!("I'm {},age {}.", name, age);
}

fn print_age((_, age): (&str, i32)) {
  println!("My age is  {}", age);
}

fn print_name((name,_): (&str, i32)) {
  println!("I am  {}", name);
}
  ```
  
   The above example is an example of a tuple (Tuple) matching, of course, it can also be other types that can be used in the let statement. The pattern matching of parameters is the same as that of let statements, and an underscore can also be used to indicate that a value is discarded.
