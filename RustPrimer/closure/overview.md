# Closure
What are closures? Let's take a look at the description on [Wikipedia][wiki]:

>In computer science, closures (English: Closure), also known as lexical closures (Lexical Closure) or function closures (function closures), are __functions that refer to free variables__. The referenced free variable will exist with the function even if it leaves the environment in which it was created. So, there is another way to say that a closure is an entity composed of a function and its associated reference environment. Closures can have multiple instances at runtime, and different reference environments and the same combination of functions can produce different instances. <br /><br />
The concept of closures appeared in the 1960s, and the earliest programming language to implement closures was Scheme. Later, closures are widely used in functional programming languages such as ML and LISP. Many imperative programming languages also support closures.

[wiki]:https://zh.wikipedia.org/wiki/%E9%97%AD%E5%8C%85_(%E8%AE%A1%E7%AE%97%E6%9C%BA%E7% A7%91%E5%AD%A6)

As you can see, the first sentence has already explained what a closure is: a closure is a function that refers to a free variable. So, a closure is a special kind of function.

In rust, functions and closures are types that implement the `Fn`, `FnMut` or `FnOnce` trait. Any type of object that implements one of these three traits is a __callable object__ that can be called in the form of `name()` like functions and closures. `()` is in rust is an operator, and operators can be overloaded in rust. Rust's operator overloading is achieved by implementing the corresponding `trait`, and the corresponding `trait` of the `()` operator is `Fn`, `FnMut` and `FnOnce`, so any implementation of these three `trait` `One of the types is actually overloading the `()` operator. For the description of `Fn`, `FnMut` and `FnOnce`, please refer to the implementation of closure in the second section.

This chapter is mainly divided into four sections:

* [Section 1 Overview](overview.md)
* [Section 2 Closure Syntax](syntax.md)
* [Section 3 Closure Implementation](implementation.md)
* [Chapter 4 Closure as parameter or return value](as_argument_return_value.md)
