# Safety

This chapter does not explain any language knowledge points, but some summary descriptions of Rust's security concepts.

Security, itself is a pretty big topic. Security itself also requires a definition of locality.

In Rust's definition, any feature that **may** cause the program to use memory incorrectly is considered **unsafe**. On the contrary, it is **safe (safe)**.

Based on this definition, the C language is basically an unsafe language (it is a collection of many unsafe features. Especially pointer related features, multi-thread related features).

This definition of Rust implies a presupposition: human nature is inherently evil. Humans are unreliable and fallible, i.e. Rust doesn't trust human implementations. At this point, the philosophy of the C language is completely opposite to it: the C language completely believes in people. At the beginning of people, people are inherently good, and they are completely controlled by people.

C is almost synonymous with unsafe by Rust's definition. However, in essence, whether a program is safe is not determined by the language in which it is developed. A program developed in C language is not necessarily an unsafe code, but relatively speaking, it needs to spend more energy on good design and long-term actual operation verification. Rust makes it relatively easy to develop safe and reliable code.

The world itself is dirty. Just as there must be a `Monad` for handling side effects in a pure functional language, Rust cannot handle all the structures and problems of the world with only a safe set of features. Therefore, in Rust, there is still an `unsafe` part. In fact, Rust's std itself is built on a lot of `unsafe` code. Therefore, the world is purely based on impurity, and "safety" is based on "unsafe".

Thus, Rust itself can be thought of as a hybrid of two programming languages: `Safe Rust` and `Unsafe Rust`.

By using only `Safe Rust`, you don't need to worry about any type safety and memory safety issues. You never have to suffer from null pointers, dangling pointers, or other possible undefined behavior.

`Unsafe Rust` only opens up the following four capabilities to programmers on all the features of `Safe Rust`:

1. Dereference raw pointers;
2. Call `unsafe` functions (including C functions, intrinsic functions, and raw allocators);
3. Implement `unsafe` traits;
4. Modify the (global) static variable.

The above four capabilities, if misused, will lead to some undefined behavior, with uncertain consequences, and it is easy to cause the program to crash.

The non-deterministic behavior defined in Rust is as follows:

1. Dereferencing a null or dangling pointer;
2. Read uninitialized memory;
3. Break the pointer renaming rules (for example, `&mut` references of the same resource cannot appear multiple times, `&mut` and `&` cannot appear at the same time);
4. Produces an invalid native value:
   - null pointer, dangling pointer;
   - the bool value is not 0 or 1;
   - undefined enumeration value;
   - char value out of range [0x0, 0xD7FF] and [0xE000, 0x10FFFF];
   - non-utf-8 strings;
5. Unwinding into other languages;
6. A data race occurs.

Some of the following situations are considered "safe" by Rust as outside the scope of security:

1. Deadlock;
2. There is a race condition;
3. Memory leaks;
4. Failed to call the destructor;
5. Integer overflow;
6. The program is interrupted;
7. Delete product database (:D);




## refer to

The following links give a more detailed explanation of security (some of them will have corresponding Chinese translations in the future).

- [Unsafe](http://doc.rust-lang.org/book/unsafe.html)
- [Meet Safe and Unsafe](http://doc.rust-lang.org/nightly/nomicon/meet-safe-and-unsafe.html)
- [How Safe and Unsafe Interact](http://doc.rust-lang.org/nightly/nomicon/safe-unsafe-meaning.html)
- [Suddenly looking back on everything is empty———Null pointer talk](http://jimhuang.cn/2015/09/12/%E8%93%A6%E7%84%B6%E5%9B%9E%E9% A6%96%E4%B8%87%E4%BA%8B%E7%A9%BA%20%E2%80%94%E2%80%94%E2%80%94%E2%80%94%E7% A9%BA%E6%8C%87%E9%92%88%E6%BC%AB%E8%B0%88/)