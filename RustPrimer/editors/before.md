# Preliminary preparation

## Download the Rust source code (for use by racer)

### Download from github

`git clone https://github.com/rust-lang/rust.git`

### Download the source code package from the official website

Download address: `https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz`

### Use rustup to download (recommended)

The biggest advantage of using rustup to obtain the source code is that you can use `rustup update` to obtain the latest version of the source code at any time, ~~ and it is very trouble-free, ~~ execute the following command to obtain the source code
```
rustup component add rust-src
```
## racer
racer is a rust auto-completion and syntax analysis tool provided by rust enthusiasts, which is used to provide basic completion functions and definition jump functions. It itself is completely written in rust, and the completion function has been relatively perfect.

We can get it as follows:

### cargo auto-installation
After version 1.5 of rust, the cargo tool that comes with the installation package already supports the cargo install command, which can help us obtain the latest version of `racer` in a simple way.

You can install the latest version of `racer` with the following command, which is currently known to be applicable on Linux, Unix and Windows

```
cargo install racer
```

### Compile and install

In fact, I recommend that qualified users install it in this way, because it is always rewarding to do it yourself. ~~(handsome and cute DCjanus expressed doubts)~~

#### Download source code

First, we need to download the source code of racer

```
git clone https://github.com/phildawes/racer.git
```

#### Compile

Then, enter the directory and compile

```
cd racer && cargo build --release
```

In this way, we will get the racer binary file in the `target/release/racer` directory

#### Setting environment variables

In order to complete the Rust standard library, racer needs to obtain the Rust source code path.

Set an environment variable named `RUST_SRC_PATH` to `[path_to_your_rust_source]/src`

Among them, `[path_to_your_rust_source]` indicates the folder where the source code is located. When using rustup to obtain the Rust source code, `[path_to_your_rust_source]` defaults to `~/.multirust/toolchains/[your-toolchain]/lib/rustlib/src/rust/src `

### test

Please reopen the terminal and enter the path before closing it.
Execute the following code:
linux:

```
./target/release/racer complete std::io::B
```

windows:

```
target\release\racer complete std::io::B
```

You will see the racer prompt, which means that the racer has completed.


## install rustfmt

`cargo install rustfmt`

## Rust Language Server (RLS)

`Rust Language Server` (hereinafter referred to as `RLS`) can provide functional support for many IDEs or editors, including but not limited to auto-completion, jump definition, rename, and jump type.

The installation steps using rustup are as follows:

1. Make sure `rustup` is the latest version
```
rustup self update
```
2. Upgrade the toolchain (it is not required to set `nightly` as the default, but you need to ensure that the `nightly` toolchain is installed)
```
rustup update nightly
```
3. Officially install RLS
```
rustup component add rls --toolchain nightly
rustup component add rust-analysis --toolchain nightly
rustup component add rust-src --toolchain nightly
```
4. Set environment variables
If the environment variable named `RUST_SRC_PATH` is not set when installing Racer, please refer to the previous section to set it.

** As of now (July 15, 2017), `RLS` is still in the alpha stage. As the project changes, the installation steps may change greatly. The RLS installation method mentioned in this article may be in a short period of time Outdated, it is recommended to follow the official installation guide for installation. **

**The project hosting address:[https://github.com/rust-lang-nursery/rls](https://github.com/rust-lang-nursery/rls)**
