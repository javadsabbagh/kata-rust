## Attributes

Attribute (Attribute) is a general feature used to express metadata. It uses the syntax of ECMA-334 (C#) to implement Attributes described in ECMA-335. Attributes can only be applied to Item (element, item),
For example `use` declarations, modules, functions, etc.

### elements

In Rust, Item is an integral part of Crate (library). it includes

* `extern crate` declaration
* `use` statement
* Module (a module is a container for Item)
* function
* `type` definition
* Structure definition
* enumeration type definition
* Constant definition
* Static variable definition
* Trait definition
* implementation (Impl)

These Items can be nested with each other, such as defining a static variable in a function, using `use` declaration or defining a structure in a module. These Items defined in a certain scope are related to you
It writes to the outermost scope to achieve the same function, but you must use the path (Path) to access these nested Items, such as `a::b::c`. But some outer Items don't allow you to use paths to
To access its sub-items, such as functions, static variables and structures defined in functions, cannot be accessed through paths.

### Attribute Syntax

The syntax of attributes is borrowed from C#, which looks like this

```rust
#[name(arg1, arg2 = "param")]
```

It is opened by a `#`, followed by a `[]`, which contains the specific content of the attribute. It can be written in the following ways:

* The attribute name represented by a single identifier, such as `#[unix]`
* A single identifier represents the property name, followed by a `=`, and then a literal (Literal), forming a key-value pair, such as `#[link(name = "openssl")]`
* A single identifier represents the property name, followed by a comma-separated list of sub-properties, such as `#[cfg(and(unix, not(windows)))]`

A `!` can also be followed by `#`, such as `#![feature(box_syntax)]`, which means that this attribute is applied to the Item where it is located. And if there is no `!`, it means that this attribute is only applied to the next Item.

For example:

```rust
// Enable the new feature box_syntax for this crate
#![feature(box_syntax)]

// This is a unit test function
#[test]
fn test_foo() {
    /* ... */
}

// Conditional compilation will only take effect when the compilation target is Linux
#[cfg(target_os="linux")]
mod bar {
    /* ... */
}

// Turn off the non_camel_case_types compilation warning for the following type definition
#[allow(non_camel_case_types)]
type int8_t = i8;
```

### Attributes applied to Crate

* `crate_name` - Specifies the name of the crate. For example, `#[crate_name = "my_crate"]` can make the compiled library name `libmy_crate.rlib`.
* `crate_type` - Specify the type of Crate, there are several options
    - `"bin"` - compiled to an executable;
    - `"lib"` - compiled as a library;
    - `"dylib"` - compiled as a dynamic link library;
    - `"staticlib"` - compiled as a statically linked library;
    - `"rlib"` - compiled into a Rust-specific library file, which is a special static link library format, which will contain some metadata for the compiler to use, and will eventually be statically linked into the object file.

   Example `#![crate_type = "dylib"]`.
* `feature` - can enable some unstable features, only available in the nightly version of the compiler.
* `no_builtins` - Remove builtins.
* `no_main`- Do not generate the `main` symbol, it will be used when the `main` function is already defined in the library you need to link.
* `no_start` - Do not link the native `native` library.
* `no_std` - Do not link with the included `std` library.
* `plugin` - Load compiler plugins, generally used to load custom compiler plugin libraries. usage is

  ```rust
  // load foo, bar two plugins
  #![plugin(foo, bar)]
  // Or pass in the necessary initialization parameters to the plugin
  #![plugin(foo(arg1, arg2))]
  ```

* `recursive_limit` - Sets the maximum recursion level at compile time. Such as automatic dereferencing, recursively defined macros, etc. The default setting is `#![recursive_limit = "64"]`

### Attributes applied to modules

* `no_implicit_prelude` - Disable automatic insertion of `use std::prelude::*`.
* `path` - Sets the file path for this `mod`.

   If `mod a;` is declared, then look for
    - `a.rs` file in this folder
    - The `a/mod.rs` file in this folder

  ```rust
  #[cfg(unix)]
  #[path = "sys/unix.rs"]
  mod sys;

  #[cfg(windows)]
  #[path = "sys/windows.rs"]
  mod sys;
  ```

### Attributes applied to functions

* `main` - Use this function as an entry function, instead of `fn main`, which will be called by the entry point.
* `plugin_registrar` - used when writing a compiler plugin, used to define the entry function of the compiler plugin.
* `start` - use this function as the entry point function (Entry Point), rewrite the `start` language item.
* `test` - indicates that this function is a unit test function and will not be compiled in a non-test environment.
* `should_panic` - Specifies that this unit test function must panic.
* `cold` - Indicates that this function is likely not to be executed, so it is treated specially when optimizing.

```rust
// use `my_main` as the main function
#[main]
fn my_main() {

}

// Use `plugin_registrar` as the entry function of this compiler plugin
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("rn", expand_rn);
}

// Use `entry_point` as the entry function, and no longer execute the initialization process in the standard library
#[start]
fn entry_point(argc: isize, argv: *const *const u8) -> isize {

}

// define a unit test
// This unit test will panic
#[test]
#[should_panic]
fn my_test() {
    panic!("I expected to be panicked");
}

// This function probably won't be executed,
// So when optimizing, change the method
#[cold]
fn unlikely_to_be_executed() {

}
```

### Attributes applied to global static variables

* `thread_local` - only available for `static mut`, indicating that this variable is thread local.

### Attributes applied to FFI

`extern` blocks can apply the following attributes

* `link_args` - Arguments given to the linker when linking, platform and implementation dependent.
* `link` - Indicates that this block needs to link a native library, it has the following parameters:
    - `name` - the name of the library, such as `libname.a` is named `name`;
    - `kind` - the type of library, which includes
        * `dylib` - dynamic link library
        * `static` - static library
        * `framework` - Framework in OS X

  ```rust
  #[link(name = "readline")]
  extern {

  }

  #[link(name = "CoreFoundation", kind = "framework")]
  extern {

  }
  ```

Inside an `extern` block, you can use

* `link_name` - specifies the name of the external function or global variable for this link;
* `linkage` - For global variables, some of LLVM's linkage types can be specified (http://llvm.org/docs/LangRef.html#linkage-types).

For `enum` types, you can use

* `repr` - currently accepts `C`, `C` means compatible with the C ABI.

```rust
#[repr(C)]
enum eType {
    Operator,
    Indicator,
}
```

For `struct` types, you can use

* `repr` - currently only accepts `C` and `packed`, `C` means the structure is compatible with C ABI, `packed` means remove padding between fields.

### Attributes for macros

* `macro_use` - export a macro defined in a module or library
    - Applied to `mod`, then export the macros defined in this module to its parent module
    - Applied to `extern crate`, it can accept a list, such as

      ```rust
      #[macro_use(debug, trace)]
      extern crate log;
      ```

       Then you can import only the macros specified in the list, otherwise all macros will be imported.

* `macro_reexport` - Applied to `extern crate`, these imported macros can be exported to other libraries for use.

* `macro_export` - Applied to macros, this macro can be exported for use by other libraries.

* `no_link` - Applied to `extern crate`, means that even if we import the library in it, don't link this library into the target file.

### Other properties

* `export_function` - For static variables or functions, specify their symbolic names in the object file.

* `link_section` - Used for static variables or functions, indicating which section they should be placed in.

* `no_mangle` - can be applied to any Item, which means cancel naming obfuscation for them, and directly write their names as symbols to the target file.

* `simd` - can be used on tuple structures, and automatically implements numerical operators, which generate corresponding SIMD instructions.

* `doc` - bind the documentation for this Item, the same function as `///`, the usage is

  ```rust
  #[doc = "This is a doc"]
  struct Foo {}
  ```

### Conditional compilation

Sometimes, we want to generate different codes for different compilation targets, such as using different code logic for Linux and Windows when writing cross-platform modules.

Conditional compilation is basically using the `cfg` attribute, just look at the example

```rust
#[cfg(target_os = "macos")]
fn cross_platform() {
    // Will only be compiled on Mac OS, including Mac OS X
}

#[cfg(target_os = "windows")]
fn cross_platform() {
    // Will only be compiled on Windows
}

// If either condition `foo` or `bar` is true, compile the following Item
#[cfg(any(foo, bar))]
fn need_foo_or_bar() {

}

// For 32-bit Unix systems
#[cfg(all(unix, target_pointer_width = "32"))]
fn on_32bit_unix() {

}

// compile if `foo` is false
#[cfg(not(foo))]
fn needs_not_foo() {

}
```

Among them, `cfg` acceptable conditions are

* `debug_assertions` - Will be true if compilation optimization is not enabled.

* `target_arch = "..."` - The CPU architecture of the target platform, including but not limited to `x86`, `x86_64`, `mips`, `powerpc`, `arm` or `aarch64`.

* `target_endian = "..."` - The endianness of the target platform, including `big` and `little`.

* `target_env = "..."` - indicates the runtime library used, such as `musl` indicates the use of the libc implementation of MUSL, `msvc` indicates the use of Microsoft's MSVC, `gnu` indicates the use of the GNU implementation.
   But this data is empty on some platforms.

* `target_family = "..."` - Indicates the class of the target operating system, such as `windows` and `unix`. This attribute can be used directly as a condition, such as `#[unix]`, `#[cfg(unix)]`.

* `target_os = "..."` - target operating system, including but not limited to `windows`, `macos`, `ios`, `linux`, `android`, `freebsd`, `dragonfly`, `bitrig` , `openbsd`, `netbsd`.

* `target_pointer_width = "..."` - The pointer width of the target platform, usually `32` or `64`.

* `target_vendor = "..."` - Vendor, eg `apple`, `pc` or `unknown` for most Linux systems.

* `test` - when unit testing is enabled (i.e. compiled with the `--test` argument, or using `cargo test`).

You can also set another condition based on one condition, using `cfg_attr`, such as

```rust
#[cfg_attr(a, b)]
```

This means that if `a` holds, then this is equivalent to `#[cfg(b)]`.

Conditional compilation attributes can only be applied to Items, what if you want to apply them to non-Items? You can use the `cfg!` macro, such as

```rust
if cfg!(target_arch = "x86") {

} else if cfg!(target_arch = "x86_64") {

} else if cfg!(target_arch = "mips") {

} else {

}
```

This method will not generate any runtime overhead, because the unfulfilled condition means that the code inside cannot be executed at all, and will be directly optimized when compiling.

### Linter parameters

The current Rust compiler has its own Linter, which can statically detect unused code, infinite loops, coding styles, etc. at compile time. Rust provides a series of properties for controlling the behavior of the Linter

* `allow(C)` - The compiler will not warn about checking errors for `C` conditions.
* `deny(C)` - The compiler will treat errors that violate the `C` condition as compilation errors.
* `forbit(C)` - behaves the same as `deny(C)`, but this will not allow others to use `allow(C)` to modify.
* `warn(C)` - The compiler will output warnings for `C` conditional check errors.

The Lint checks supported by the compiler can be viewed by executing `rustc -W help`.

### Inline parameters

The inline function suggests that the compiler can consider copying the entire function to the caller's function body instead of generating a `call` instruction to call it. This optimization is very useful for short functions and is good for performance.

The compiler will judge whether a function should be inlined according to some default conditions. If a function that should not be inlined is inlined, it will actually make the entire program slower.

Optional attributes are:

* `#[inline]` - Suggests the compiler to inline this function
* `#[inline(always)]` - requires the compiler to inline this function
* `#[inline(never)]` - asks the compiler not to inline this function

Inlining causes code in one library to be inserted into another library.

### Automatically implement Trait

The compiler provides a compiler plug-in called `derive`, which can help you generate some code to implement (impl) some specific Trait, such as

```rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
```

The compiler will automatically generate the following code for you

```rust
impl<T: PartialEq> PartialEq for Foo<T> {
    fn eq(&self, other: &Foo<T>) -> bool {
        self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &Foo<T>) -> bool {
        self.a != other.a || self.b != other.b
    }
}

impl<T: Clone> Clone for Foo<T> {
    fn clone(&self) -> Foo<T> {
        Foo {
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}
```

Currently `derive` only supports some traits in the standard library.

### Compiler Features

In the unstable version of the Rust compiler, you can use some unstable features, such as some new features that are still under discussion, features that are being implemented, etc. The Rust compiler provides an attribute `feature` applied to Crates to enable these unstable features, such as

```rust
#![feature(advanced_slice_patterns, box_syntax, asm)]
```

The specific available compiler features will vary with the release of the compiler version, please read the official documentation for details.
