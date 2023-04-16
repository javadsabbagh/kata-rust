# Ownership system

## Overview

The Ownership System is one of the most fundamental, unique and important features of the Rust language.

The goal Rust pursues is memory safety and operating efficiency, but it does not have the memory garbage collection mechanism GC of languages such as golang, java, and python.

The Rust language claims that as long as the compilation is passed, it will not crash (memory safety); it has zero or very small runtime overhead (running efficiency). These advantages also benefit from Rust's ownership system.

The ownership system consists of three important components:

- **Ownership** (ownership)
- **Borrowing**
- **Lifetimes** (lifetime)

These three features are related to each other, and will be fully explained in the following chapters.

> **Hint:**
> Rust’s ownership system may be difficult for many beginners to understand. Rust’s memory check is done at the compilation stage. This check is very rigorous, so it may be difficult for beginners to compile code at the beginning Compilation passed.

> But don't be afraid :), once you get familiar with it you will love it and benefit a lot in future programming. The ownership system requires readers to slowly understand its mysteries, and you can also refer to official documents during the learning process.
