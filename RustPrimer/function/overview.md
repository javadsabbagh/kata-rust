# function
   Although rust is a multi-paradigm programming language, rust's programming style is more functional, and functions are "first-class citizens" in rust - first-class type. This means that the function can be passed as data in the program, such as: as a parameter of the function. Like C and C++, the rust program also has a unique program entry-main function. The main function of rust is as follows:
  
  ```rust
fn main() {
  //statements
}
  ```
  
   Rust uses the `fn` keyword to declare and define functions. The `fn` keyword is separated by a space followed by the function name, the function name is followed by a parenthesis, and the function parameters are defined in the parentheses. rust uses the `snake_case` style to name functions, that is, all letters are lowercase and use underscores to separate words, such as: `foo_bar`. If the function has a return value, add the arrow __->__ after the parentheses, and add the type of the return value after the arrow.

   In this chapter we will learn the following knowledge related to functions:
  1. [Function Arguments](arguement.md)
  2. [Function return value](return_value.md)
  3. [Statements and expressions](statement_expression.md)
  4. [Higher order function](higher_order_function.md)

> ### Note: All the examples in this chapter have been compiled under rustc1.4, and all the compilation errors described in the examples are given by the rustc1.4 version.
