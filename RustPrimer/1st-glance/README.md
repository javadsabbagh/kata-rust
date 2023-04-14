Rust is a systems-level programming language designed to be memory and thread-safe and prevent segmentation faults. As a system-level programming language, its basic idea is "zero-overhead abstraction". In theory, its speed is at the same level as C/C++.

Rust can be classified as a general-purpose, multi-paradigm, compiled programming language, similar to C or C++. Unlike those two programming languages, Rust is thread-safe!

The goal of the Rust programming language is to create a highly safe and concurrent software system. It emphasizes security, concurrency, and memory control. Although Rust borrows syntax from C and C++, it does not allow null pointers and dangling pointers, which are a source of system crashes, memory leaks, and unsafe code in C and C++.

Rust has common control structures such as if else and looping statements for and while. As in C and C++ style programming languages, sections of code are enclosed in curly braces.

Rust uses implementations, traits, and structured types instead of classes. This point is quite different from inheritance-based OO languages C++ and Java. It is closer to functional languages such as Ocaml and Haskell.

Rust achieves memory safety without the overhead of implementing automatic garbage collectors in the .NET and Java programming languages, through ownership/borrowing mechanisms, lifetimes, and the type system.

Here is an example code snippet, the classic Hello World application:

``` rust
fn main() {
  println!("hello, world");
}
```

Popular programming languages that have influenced Rust include C, C++, C#, Erlang, Haskell, OCaml, Ruby, Scheme, and Swift, among others. Rust has also influenced C# 7, Elm, Idris, Swift.

Rust provides an installer, you just need to download it from the official website and run the installer on the corresponding operating system. The installer supports 32-bit and 64-bit CPU architectures on Windows, Mac and Linux (via scripting), under Apache License 2.0 or MIT Licenses.

Rust runs on the following operating systems: Linux, OS X, Windows, FreeBSD, Android, iOS.

A brief mention of Rust's history. Rust was originally a personal project of Mozilla employee Graydon Hoare, started in 2009, supported by Mozilla Research Institute, and the project was announced in 2010. Bootstrap achieved between 2010 and 2011. Since then, Rust has gone through a lot of design changes and iterations (very hard) before finally releasing version 1.0 on May 15, 2015. During this research and development process, Rust established a strong and active community and formed a complete and stable project contribution mechanism (this is the real scary thing). Rust is now maintained by the Rust project developer community (https://github.com/rust-lang/rust).

Since the release of 1.0 in May 2015, a large number of excellent projects have emerged (you can search Rust on github to find them), and large companies have gradually actively participated in the development of Rust applications and gave back to the open source community.

This book (RustPrimer) aims to provide a correct, up-to-date and easy-to-understand Chinese tutorial for beginners of Chinese Rustaceans. This book will always be perfected and followed up, and will never stop.

This book is the result of the joint efforts of the entire Rust Chinese community. Among them, the Rustaceans who participated in the writing and editing of this book are (in no particular order):

- [daogangtang（Mike猫）](https://github.com/daogangtang)
- [wayslog（猫猫反抗团团长）](https://github.com/wayslog)
- [marvin-min](https://github.com/marvin-min)
- [tiansiyuan](https://github.com/tiansiyuan)
- [marvinguo](https://github.com/marvinguo)
- ee0703
- fuyingfuying
- qdao
- JohnSmithX
- [stormgbs (AX) ](https://github.com/stormgbs)
- tennix
- anzhihun
- zonyitoo（Elton, e猫）
- 42
- [Naupio（N猫）](https://github.com/Naupio)
- F001（失落的神喵）
- wangyu190810
- domty
- [MarisaKirisame（帅气可爱魔理沙）](https://github.com/MarisaKirisame)
- [Liqueur Librazy](https://github.com/Librazy)
- [Knight42](https://github.com/knight42)
- [Ryan Kung](https://github.com/ryankung)
- lambdaplus
- doomsplayer
- lucklove
- veekxt
- lk-chen
- RyanKung
- arrowrowe
- marvin-min
- ghKelo
- wy193777
- domty
- xusss
- wangyu190810
- nextzhou
- zhongke
- [ryuki](https://github.com/3442853561)
- codeworm96
- anzhihun
- lidashuang
- sceext2
- loggerhead
- twq0076262
- passchaos
- yyrust
- markgeek
- ts25504
- overvenus
- Akagi201
- theJian
- jqs7
- ahjdzx
- chareice
- chenshaobo
- marvinguo
- izgzhen
- ziqin
- peng1999

wait. Here, I would like to express my respect and thanks to them for their hard work and selfless dedication!

Have fun programming in Rust!
