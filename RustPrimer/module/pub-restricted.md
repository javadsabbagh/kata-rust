# pub restricted

## Overview

This is a syntax added in rust1.18. In previous versions, `item` only had two categories: `pub`/non-`pub`, and the syntax of pub restricted was used to extend the use of `pub` to enable it to specify the desired scope\(visible scope\ ), see RFC [1422-pub-restricted.md](https://github.com/rust-lang/rfcs/blob/master/text/1422-pub-restricted.md) for details.

In Rust, `crate` is a module tree, you can use the expression `pub(crate) item;` to restrict `item` to be only available in the current `crate`, and in other subtrees of the current `crate`, you can pass `use + path` syntax to refer to `item`.

## Design Motivation

Before Rust1.18, if we want to design an item `x` that can be used in multiple places, there are two ways:

* Define a non `pub` item in the root directory;
* Define a `pub` item in the submodule, and reference this item to the root directory through `use`.

However, sometimes neither of these methods is what we want. In some cases, we want the item to be visible for certain modules but not for other modules.

Let's look at an example:

```Rust
// Intent: `a` exports `I`, `bar`, and `foo`, but nothing else.
pub mod a {
    pub const I: i32 = 3;

    // `semisecret` will be used "many" places within `a`, but
    // is not meant to be exposed outside of `a`.
    fn semisecret(x: i32) -> i32  { use self::b::c::J; x + J }

    pub fn bar(z: i32) -> i32 { semisecret(I) * z }
    pub fn foo(y: i32) -> i32 { semisecret(I) + y }

    mod b {
        mod c {
            const J: i32 = 4; // J is meant to be hidden from the outside world.
        }
    }
}
```

This code fails to compile because `J` is not accessible outside of `mod c`, and `fn semisecret` tries to access `J` in `mod a`.

Before rust1.18, the correct way to keep `J` private and enable `a` to use `fn semisecret` is to move `fn semisecret` to `mod c`, and `pub` it, and then according to Need to be able to re-export `semisecret`. (If you don't need to keep `J` private, you can `pub` it, then you can `pub use self::c::J` in `b` or directly `pub c`)

```Rust
// Intent: `a` exports `I`, `bar`, and `foo`, but nothing else.
pub mod a {
    pub const I: i32 = 3;

    // `semisecret` will be used "many" places within `a`, but
    // is not meant to be exposed outside of `a`.
    // (If we put `pub use` here, then *anyone* could access it.)
    use self::b::semisecret;

    pub fn bar(z: i32) -> i32 { semisecret(I) * z }
    pub fn foo(y: i32) -> i32 { semisecret(I) + y }

    mod b {
        pub use self::c::semisecret;
        mod c {
            const J: i32 = 4; // J is meant to be hidden from the outside world.
            pub fn semisecret(x: i32) -> i32  { x + J }
        }
    }
}
```

This situation works fine, but there is a serious problem: no one can clearly explain where `pub fn semisecret` is used, it needs to be judged by context:

1. All modules that can access `semisecret`;
2. In all modules that can access `semisecret`, whether there is a re-export of `semisecret`;

At the same time, if `pub use self::b::semisecret` is used in `a`, then everyone can access `fn semisecret` through `use`, but in fact, this function only needs to be accessed by `mod a` That's it.

## Use of pub restricted

### Syntax

old:

    VISIBILITY ::= <empty> | `pub`

new:

    VISIBILITY ::= <empty> | `pub` | `pub` `(` USE_PATH `)` | `pub` `(` `crate` `)`

pub\(restriction\) means to limit the visibility (scope) of the definition of item, method, field, etc.

The visible range (scope) is divided into all crates \(unlimited\), the current crate, and the absolute paths of submodules in the current crate. A restricted thing cannot be used directly outside of its restricted scope.

* `pub` unspecified means unlimited;
* `pub(crate)` current crate is valid;
* `pub(in <path>)` is available in the module indicated by `<path>`.

### modify the example

```Rust
// Intent: `a` exports `I`, `bar`, and `foo`, but nothing else.
pub mod a {
     pub const I: i32 = 3;

     // `semisecret` will be used "many" places within `a`, but
     // is not meant to be exposed outside of `a`.
     // (`pub use` would be *rejected*; see Note 1 below)
     use self::b::semisecret;

     pub fn bar(z: i32) -> i32 { semisecret(I) * z }
     pub fn foo(y: i32) -> i32 { semisecret(I) + y }

     mod b {
         pub(in a) use self::c::semisecret;
         mod c {
             const J: i32 = 4; // J is meant to be hidden from the outside world.

             // `pub(in a)` means "usable within hierarchy of `mod a`, but not
             // elsewhere."
             pub(in a) fn semisecret(x: i32) -> i32 { x + J }
         }
     }
}
```

Note 1: If changed to the following method, the compiler will report an error:

```Rust
pub mod a { [...] pub use self::b::semisecret; [...] }
```

Because `pub(in a) fn semisecret` indicates that this function can only be used in `a`, and `pub` is not allowed to go out of the scope of `a`.

### Limit field example

```Rust
mod a {
     #[derive(Default)]
     struct Priv(i32);

     pub mod b {
         use a::Priv as Priv_a;

         #[derive(Default)]
         pub struct F {
             pub x: i32,
                    y: Priv_a,
             pub(in a) z: Priv_a,
         }

         #[derive(Default)]
         pub struct G(pub i32, Priv_a, pub(in a) Priv_a);

         // ... accesses to F.{x,y,z} ...
         // ... accesses to G.{0,1,2} ...
     }
     // ... accesses to F.{x,z} ...
     // ... accesses to G.{0,2} ...
}

mod k {
     use a::b::{F, G};
     // ... accesses to F and F.x ...
     // ... accesses to G and G.0 ...
}
```

### Crate limit example

Crate `c1`:

```Rust
pub mod a {
     struct Priv(i32);

     pub(crate) struct R { pub y: i32, z: Priv } // ok: field allowed to be more public
     pub struct S { pub y: i32, z: Priv }

     pub fn to_r_bad(s: S) -> R { ... } //~ ERROR: `R` restricted solely to this crate

     pub(crate) fn to_r(s: S) -> R { R { y: s.y, z: s.z } } // ok: restricted to crate
}

use a::{R, S}; // ok: `a::R` and `a::S` are both visible

pub use a::R as ReexportAttempt; //~ ERROR: `a::R` restricted solely to this crate
```

Crate `c2`:

```Rust
extern crate c1;

use c1::a::S; // ok: `S` is unrestricted

use c1::a::R; //~ ERROR: `c1::a::R` not visible outside of its crate
```
