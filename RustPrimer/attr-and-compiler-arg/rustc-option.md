# Compiler parameters

This chapter describes the parameters of the Rust compiler.

The name of the Rust compiler program is `rustc`, and using it is simple:

```bash
$ rustc [OPTIONS] INPUT
```

Among them, `[OPTIONS]` represents the compilation parameters, and `INPUT` represents the input file. The compilation parameters have the following options:

* `-h, --help` - output help information to standard output;

* `--cfg SPEC` - Pass in custom conditional compilation parameters, use methods such as

  ```rust
  fn main() {
      if cfg!(hello) {
          println!("world!");
      }
  }
  ```

   As shown in the above example, if `cfg!(hello)` is established, the running program will output `"world"` to the standard output. We save this file as `hello.rs` and compile it

  ```bash
  $ rustc --cfg hello hello.rs
  ```

   Run it and you'll see `world!` printed to the screen.

* `-L [KIND=]PATH` - Add a folder to the link path, and you can specify the type of this path (Kind), these types include
  - `dependency` - Find dependent files in this path, such as `mod`;
  - `crate` - only find libraries defined in `extern crate` in this path;
  - `native` - only find Native libraries in this path;
  - `framework` - only useful under OS X, only find Framework under this path;
  - `all` - the default option.

* `-l [KIND=]NAME` - link a library that can specify the type (Kind)
  - `static` - static library;
  - `dylib` - dynamic library;
  - `framework` - Framework for OS X.

   If not passed, defaults to `dylib`.

   Here is an example of how to manually link a library, we first create a file called `myhello.rs`, and write a function in it

  ```rust
  // myhello.rs

  /// This function simply prints Hello World! to the label output
  /// Don't forget to mark it as pub.
  pub fn print_hello() {
      println!("Hello World!");
  }
  ```

   Then compile this file into a static library, `libmyhello.a`

  ```bash
  $ rustc --crate-type staticlib myhello.rs
  ```

   Then create a `main.rs`, link this library and print "Hello World!"

  ```rust
  // main.rs

  // Specify link library myhello
  extern crate myhello;

  fn main() {
      // call library function
      myhello::print_hello();
  }
  ```

   compile `main.rs`

  ```bash
  $ rustc -L. -lmyhello main.rs
  ```

   Run `main`, you will see the screen output "Hello World!".

* `--crate-type` - specifies the type of compiled output, its parameters include
  - `bin` - binary executable
  - `lib` - compiled as a library
  - `rlib` - Rust library
  - `dylib` - dynamic link library
  - `staticlib` - Statically linked library

* `--crate-name` - Specify the name of this Crate, the default is the file name, such as `main.rs` when compiled into an executable file, the default is `main`, but you can specify it as `foo`

  ```bash
  $ rustc --crate-name foo main.rs
  ```

   will output the `foo` executable.

* `--emit` - Specify compiler output. The compiler outputs an executable or library by default, but you can choose to output something else for Debug

  - `asm` - output assembly
  - `llvm-bc` - [LLVM Bitcode](http://llvm.org/docs/BitCodeFormat.html);
  - `llvm-ir` - [LLVM IR](http://llvm.org/docs/LangRef.html), namely LLVM intermediate code (LLVM Intermediate Representation);
  - `obj` - Object File (that is, `*.o` files);
  - `link` - this is to be used in combination with other `--emit` parameters, it will execute the Linker and output the result;
  - `dep-info` - file dependencies (for Debug, similar to Makefile dependencies).

   The above parameters can be used at the same time, separated by commas, such as

  ```bash
  $ rustc --emit asm,llvm-ir,obj main.rs
  ```

   At the same time, you can add a `=PATH` at the end to specify the output to a specific file, such as

  ```bash
  $ rustc --emit asm=output.S,llvm-ir=output.ir main.rs
  ```

   This will generate the assembly into the `output.S` file and the LLVM intermediate code into the `output.ir`.

* `--print` - print some information, parameters have
  - `crate-name` - compile target name;
  - `file-names` - compiled file names;
  - `sysroot` - Print the address of the root directory of the Rust toolchain.

* `-g` - Save symbols in object file, this parameter is equivalent to `-C debuginfo=2`.

* `-O` - Enable optimization, this parameter is equivalent to `-C opt-level=2`.

* `-o FILENAME` - Specifies the output filename, also applies to the output of `--emit`.

* `--out-dir DIR` - Specifies the output folder, the default is the current folder, and the `-o` configuration will be ignored.

* `--explain OPT` - explain a compile error, e.g.

   If you write a `main.rs`, use an undefined variable `f`

  ```rust
  fn main() {
      f
  }
  ```

   When compiling it, the compiler complains:

  ```
  main.rs:2:5: 2:6 error: unresolved name `f` [E0425]
  main.rs:2     f
                ^
  main.rs:2:5: 2:6 help: run `rustc --explain E0425` to see a detailed explanation
  error: aborting due to previous error
  ```

   Although the error is already obvious, you can also ask the compiler to explain what is the `E0425` error:

  ```bash
  $ rustc --explain E0425
  // Instructions printed by the compiler
  ```

* `--test` - compile to a unit test executable

* `--target TRIPLE` - specify the target platform, the basic format is `cpu-manufacturer-kernel[-os]`, for example

  ```bash
  ## 64-bit OS X
  $ rustc --target x86_64-apple-darwin
  ```

* `-W help` - Print all configurable options and default values for the Linter.

* `-W OPT, --warn OPT` - set a linter option to Warning.
* `-A OPT, --allow OPT` - Set a linter option to Allow.
* `-D OPT, --deny OPT` - Set a linter option to Deny.
* `-F OPT, --forbit OPT` - Set a linter option to Forbit.

* `-C FLAG[=VAL], --codegen FLAG[=VAL]` - related parameters generated by the target code, you can use `-C help` to view the configuration, a few worth noting are
  - `linker=val` - specify the linker;
  - `linker-args=val` - specify linker arguments;
  - `prefer-dynamic` - the default Rust compilation is static linking, selecting this configuration will change to dynamic linking;
  - `debug-info=level` - Debug information level, `0` = no generation, `1` = only generate file line number table, `2` = generate all;
  - `opt-level=val` - optimization level, optional `0-3`;
  - `debug_assertion` - explicitly enable `cfg(debug_assertion)` conditional.

* `-V, --version` - print compiler version number.

* `-v, --verbose` - enable verbose mode (print compiler execution log).

* `--extern NAME=PATH` - Used to specify the name and path of an external Rust library (`*.rlib`), the name should be the same as specified in `extern crate`.

* `--sysroot PATH` - Specifies the toolchain root directory.

* `-Z flag` - Parameters for compiler debugging, you can use `-Z help` to view the available parameters.

* `--color auto|always|never` - add color to the log on output
  - `auto` - Automatically choose whether to add or not, if the output target is a virtual terminal (TTY), add it, otherwise don't add it;
  - `always` - add me!
  - `never` - Do you dare to add?
