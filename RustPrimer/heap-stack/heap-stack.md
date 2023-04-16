# Heap & Stack

## Introduction

Heap and stack are the most basic concepts in computers, but if you have been using high-level languages such as Python/Ruby/PHP/Java, you may not understand heap and stack very well. Of course, the stack (Stack) here is not It is not a concept in a data structure, but an abstraction of memory by a computer. In contrast, languages such as C/C++/Rust must have a good understanding of the concepts of heap and stack to write correct programs. The reason for this difference is that their memory management methods are different. Language programs such as Python The runtime will run the garbage collector at the same time. The garbage collector and the user program are executed in parallel or interleaved. The garbage collector will automatically release the memory space that is no longer used, while C/C++/Rust does not have a garbage collector.

The operating system maps physical memory into a virtual address space, and the virtual address space seen by the program at startup is a complete continuous memory.

The stack memory grows downward from the high address, and the stack memory allocation is continuous. The general operating system has a limit on the stack memory size. On Linux/Unix type systems, the maximum stack space size can be set through ulimit, so it cannot be created in C language. An array of arbitrary length. In Rust, a temporary stack space is created when a function is called. After the call is over, Rust will let the objects in this stack space automatically enter the `Drop` process, and finally the top pointer of the stack is automatically moved to the top of the previous call stack, without the need for the programmer to manually Intervention, so stack memory application and release is very efficient.

In contrast, the memory on the heap grows upwards from the low address. The heap memory is usually only limited by the physical memory, and it is usually discontinuous. It is usually applied and released manually by the programmer. If you want to apply for a piece of continuous memory, the operating system It is necessary to find an unused continuous memory space of sufficient size in the heap, so its efficiency is much lower than that of the stack, especially if there is a large amount of discontinuous memory on the heap. In addition, the memory must be manually released by the programmer after use, otherwise there will be a memory leak, which has a great impact on programs that need to run for a long time (such as daemon processes).

## Heap and stack in Rust

Since the function stack will be destroyed after the function is executed, the variables stored on the stack cannot be passed between functions, which also means that the function cannot return the reference of the variables on the stack, and this is usually a common mistake made by C/C++ novices . The compiler in Rust will check out this kind of error, and the error message is generally `xxx does not live long enough`, see the following example


```rust
fn main() {
     let b = foo("world");
     println!("{}", b);
}

fn foo(x: &str) -> &str {
     let a = "Hello, ".to_string() + x;
     &a
}
```

The reason for writing this way is that many people think that they can directly copy the reference of the string `a` to avoid copying the entire string, but the result is a compilation error of `a does not live long enough`. Because a variable temporarily created in the function stack is referenced, the function stack will be destroyed after the function call ends, so the returned reference becomes meaningless, pointing to a variable that does not exist. Compared with C/C++, using Rust will be much luckier, because if you write the above program in C/C++, the compiler will silently let you pass it until it runs and will not give you an error.

In fact, since `a` itself is of String type and is stored using the heap, it can be returned directly, and it still exists after the function stack is destroyed when the function returns. Also the following code in Rust is actually just a shallow copy.

```rust
fn main() {
     let b = foo("world");
     println!("{}", b);
}

fn foo(x: &str) -> String {
     let a = "Hello, ".to_string() + x;
     a
}
```

Rust uses the stack to store variables by default, and the memory allocation on the stack is continuous, so you must know the memory space occupied by variables before compiling, so that the compiler can arrange the memory layout reasonably.

## Box

In C, the memory space on the heap is managed manually through malloc/free, while in Rust there are many ways, the most commonly used one is Box, which can apply for a piece of memory space on the heap through `Box::new()`, Unlike C, where the space on the heap needs to be released manually by calling `free`, in Rust, the compiler uses lifetime to analyze the lifetime of the heap memory at compile time, and automatically inserts `free` at the end of the lifetime. The current bottom layer of Rust, namely Box, calls jemalloc for memory management, so the space on the heap does not need to be manually managed and released by programmers. Many times when you are tortured to death by the compiler, those `borrow`, `move`, `lifetime` errors are actually the compiler teaching you about memory layout and using lifetime rules to control memory. This set of rules is not difficult to say, and it is not easy to say that it is simple. When writing programs in other languages ​​in the past, you don’t care about memory. You may really be tortured to death when you first write it. But once you are familiar with this set of rules, you can master memory layout Once you understand it, you will be able to write programs with the help of the compiler's prompts. This set of rules is the result of research in the theoretical circles and practiced on the Rust compiler.

Objects in most object-oriented languages with GC are implemented with the help of boxes, such as the common dynamic languages Python/Ruby/JavaScript, etc., which declare that "everything is an object (Everything is an object)", the so-called object Basically all boxed value.

Compared with unboxed, the boxed value takes up more memory space. At the same time, when accessing the value, it needs to be unboxed first, that is, the pointer is dereferenced and then the actual stored value is obtained, so the memory access overhead will also be larger. Since boxed values are space and time consuming, why bother? Because through the box, all objects seem to be stored with the same size, because only one pointer needs to be stored, and the application can treat all kinds of values ​​equally, regardless of the actual storage value, how to apply and Release the corresponding resources.

Box is the memory allocated on the heap, through `Box::new()` will create a heap space and return a pointer to the heap space

The `box` keyword is introduced in the nightly version, which can be used to replace `Box::new()` to apply for a heap space, and can also be used in pattern matching

```rust
#![feature(box_syntax, box_patterns)]
fn main() {
   let boxed = Some(box 5);
   match boxed {
       Some(box unboxed) => println!("Some {}", unboxed),
       None => println!("None"),
   }
}
```

Let's look at an example and compare the memory layout of `Vec<i32>` and `Vec<Box<i32>>`. These two pictures are from [Stack Overflow](http://stackoverflow.com/questions/21066133/what- is-the-difference-between-veci32-and-vecboxi32/21067103#21067103), from these two memory distribution diagrams, you can clearly and intuitively see how the Box is stored


```
Vec<i32>

(stack)    (heap)
┌──────┐   ┌───┐
│ vec1 │──→│ 1 │
└──────┘   ├───┤
           │ 2 │
           ├───┤
           │ 3 │
           ├───┤
           │ 4 │
           └───┘
```


```
Vec<Box<i32>>

(stack)    (heap)   ┌───┐
┌──────┐   ┌───┐ ┌─→│ 1 │
│ vec2 │──→│   │─┘  └───┘
└──────┘   ├───┤    ┌───┐
           │   │───→│ 2 │
           ├───┤    └───┘
           │   │─┐  ┌───┐
           ├───┤ └─→│ 3 │
           │   │─┐  └───┘
           └───┘ │  ┌───┐
                 └─→│ 4 │
                    └───┘
```

In some languages, there are data structures that look like both arrays and lists. For example, List in python is actually similar to `Vec<Box<i32>>`, except that i32 is replaced by any type. In terms of operational efficiency It is more efficient than a simple List and more flexible than an array.

Generally speaking, data types whose size cannot be determined during compilation need to use heap memory, because the compiler cannot allocate memory with an unknown size during compilation on the stack, so types of memory such as String and Vec are actually allocated on the heap Up. In other words, we can easily move a Vec out of scope without worrying about consumption, since the data won't actually be copied.

In addition, when you need to return a shallow copy variable from a function, you also need to use heap memory instead of directly returning a reference to a variable defined inside the function.
