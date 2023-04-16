# Packages and modules

## package (crate)

In Rust, a crate is an independent compilable unit. Specifically, it is one or a batch of files (if it is a batch of files, then one file is the entry of this crate). After it is compiled, it will generate an executable file or a library correspondingly.

Execute `cargo new foo`, you will get the following directory hierarchy:

```
foo
├── Cargo.toml
└── src
     └── lib.rs
```

Here, `lib.rs` is a crate (entry), which is compiled into a library. A project can contain more than one crate, and this project only has one.

Execute `cargo new --bin bar`, you will get the following directory hierarchy:

```
bar
├── Cargo.toml
└── src
     └── main.rs
```

Here, `main.rs` is a crate (entry), which is compiled into an executable file.


## module

Rust provides a keyword `mod`, which can define a module in one file, or reference a module in another file.

Some key points about modules:

1. In each crate, an implicit root module is implemented by default;
2. The naming style of the module is also `lower_snake_case`, which is the same as other Rust identifiers;
3. Modules can be nested;
4. Any legal Rust code can be written in the module;

### Define a module in the file

For example, in the above `lib.rs`, we write the following code:

```rust
mod aaa {
    const X: i32 = 10;

    fn print_aaa() {
        println!("{}", 42);
    }
}
```

We can continue to write the following code:

```rust
mod aaa {
    const X: i32 = 10;

    fn print_aaa() {
        println!("{}", 42);
    }

    mod BBB {
        fn print_bbb() {
            println!("{}", 37);
        }
    }
}
```

You can also continue to write:

```rust
mod aaa {
    const X: i32 = 10;

    fn print_aaa() {
        println!("{}", 42);
    }

    mod bbb {
        fn print_bbb() {
            println!("{}", 37);
        }
    }
}

mod ccc {
    fn print_ccc() {
        println!("{}", 25);
    }

}

```

### Module Visibility

We wrote some modules earlier, but in fact, we wrote those modules, which currently have no effect. The purpose of writing a module is to separate logical blocks, and the second is to provide appropriate functions or objects for external access. The content in the module is private by default and can only be accessed inside the module.

In order to make the item in the module available externally, the `pub` keyword needs to be used. For external references, use the `use` keyword. For example:

```rust
mod ccc {
    pub fn print_ccc() {
        println!("{}", 25);
    }
}

fn main() {
    use ccc::print_ccc;

    print_ccc();
    // or
    ccc::print_ccc();
}
```

The rule is very simple, an item (function, binding, Trait, etc.), if `pub` is added in front of it, then it becomes visible (accessed, called) to the outside world.


### Reference external file module

Normally, we would write module content in a separate file, and then use the `mod` keyword to load that file as our module.

For example, we created a new file `aaa.rs` under `src`. The directory structure now looks like this:

```
foo
├── Cargo.toml
└── src
    └── aaa.rs
    └── main.rs
```

In `aaa.rs`, we write:

```rust
pub fn print_aaa() {
    println!("{}", 25);
}
```

In `main.rs`, write:

```rust
mod aaa;

use self::aaa::print_aaa;

fn main () {
     print_aaa();
}
```

After compilation, an executable file is generated.

Careful friends will find that in `aaa.rs`, `mod xxx {}` is not used to wrap it up, because `mod xxx;` is equivalent to wrapping the `xxx.rs` file with `mod xxx {}` up. Beginners tend to add an extra layer, beware.


### Hierarchical relationship of multi-file modules

Rust's modules support a hierarchy, but the hierarchy itself is decoupled from the filesystem directory hierarchy.

`mod xxx;` The `xxx` cannot contain a `::` sign. That is to say, in this expression form, it is impossible to refer to the modules under the multi-layer structure. That is, you cannot directly use `mod a::b::c::d;` to refer to `a/b/c/d.rs` this module.

Then, Rust's multi-layer modules follow the following two rules:

1. First look for `xxx.rs` files
     1. `mod xxx;` in `main.rs`, `lib.rs`, and `mod.rs` will search for `xxx.rs` files in the same directory by default;
     2. `mod xxx;` in other files `yyy.rs` will first search for `xxx.rs` files in the `yyy` directory of the same level directory by default;
2. If `xxx.rs` does not exist, then look for the `xxx/mod.rs` file, that is, the `mod.rs` file under the `xxx` directory.

In the above two cases, after being loaded into a module, the effect is the same. Based on these two rules, Rust implements the loading of modules in deep directories through iterative use and the `pub` keyword;

Let's take an example, now we have built a test project, the directory structure is as follows:

```
src
├── a
│ ├── b
│ │ ├── c
│ │ │ ├── d.rs
│ │ │ └── mod.rs
│ │ └── mod.rs
│ └── mod.rs
└── main.rs

```

`a/b/c/d.rs` file content:

```rust
pub fn print_ddd() {
     println!("i am ddd.");
}
```

`a/b/c/mod.rs` file content:

```rust
pub mod d;
```

`a/b/mod.rs` file content:

```rust
pub mod c;
```

`a/mod.rs` file content:

```rust
pub mod b;
```

`main.rs` file content:

```rust
mod a;

use self::a::b::c::d;

fn main() {
     d::print_ddd();
}

```
The output is: `i am ddd.`

After carefully understanding this example, you will understand the usage of Rust's hierarchical structure module.

As for why Rust is designed this way, there are several reasons:

1. The design of Rust's own module is decoupled from the file system directory of the operating system, because Rust itself can be used for the development of the operating system;
2. A file in Rust can contain multiple modules, directly mapping `a::b::c::d` to `a/b/c/d.rs` will cause some ambiguity;
3. Starting from the standpoint of safety and explicitness, Rust requires every node in the reference path to be a valid module. For example, if `d` is a valid module in the above example, then `c, b, a` are valid modules respectively and can be referenced individually.


### path

As we mentioned earlier, a crate is an independent compilable unit. It has an entry file, which is the module root path of this crate (which may contain several modules). The reference of the whole module forms a chain, and each module can be represented by a precise path (for example: `a::b::c::d`);

Similar to the concept of the file system, the module path also has the concept of relative path and absolute path. For this, Rust provides two keywords `self` and `super`.

`self` in the path has two meanings:

1. `use self::xxx` means to load `xxx` in the current module. In this case self can be omitted;
2. `use xxx::{self, yyy}` means to load the module `xxx` itself under the current path, and `yyy` under the module `xxx`;

`super` indicates that the upper level path of the current module path can be understood as the parent module.
```rust
use super::xxx;
```
Indicates the reference to `xxx` in the parent module.

Additionally, there is a special path form:
```rust
::xxx::yyy
```
It means to refer to `xxx::yyy` under the root path, which refers to the root path of the current crate.

`*` symbols in paths:
```rust
use xxx::*;
```
Indicates all visible items (items marked with pub) imported under the `xxx` module.

### Re-exporting

We can combine `pub use` to achieve `Re-exporting`. `Re-exporting` literally means `re-exporting`. It means this, to export the deep item to the upper directory, so that it is more convenient to call. This technique will be used extensively in interface design.

Or take the example of `a::b::c::d` above. We are in `main.rs`, to call `d`, we have to use `use a::b::c::d;` to call. And if we modify the `a/mod.rs` file to:
`a/mod.rs` file content:

```rust
pub mod b;
pub use b::c::d;
```

Then, in `main.rs`, we can use `use a::d;` to call. From this example, I don't think it's much more convenient. But if there is a large amount of content in a developed library, and it is in modules of different levels. Then, by uniformly exporting to one place, it will greatly facilitate the interface users.

### Load external crate

What we have mentioned above are all technologies in the current crate. When we are actually developing, we will use a lot of external libraries. External libraries are accessed via

```rust
extern crate xxx;
```

Introduced in this way.

Note: To make the above references effective, you must also add `xxx="version num"` in the `dependencies` section of `Cargo.toml`, see `Cargo Project Management` for details.

After importing, it is equivalent to introducing a symbol `xxx`, and later you can directly use this `xxx` as the root to refer to the item in this crate:

```rust
extern crate xxx;

use xxx::yyy::zzz;
```

When imported, it can be renamed with the `as` keyword.

```rust
extern crate xxx as foo;

use foo::yyy::zzz;
```
