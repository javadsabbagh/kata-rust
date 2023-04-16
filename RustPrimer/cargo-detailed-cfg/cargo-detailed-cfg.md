Hello guys, we meet again. Before Chapter 5, we discussed some common basic skills of cargo. Through the study of Chapter 5, you can basically solve most of the problems encountered in daily project development. But in fact, the functions that cargo provides to us are not limited to this. I just want to say one word: cargo is very good and powerful, and far more powerful than you think.
This chapter will delve into some details of cargo, including the following aspects:

- Project version declaration and management based on semantic version
- detailed reference of cargo's toml description file configuration fields

# Project version declaration and management based on semantic version
When we use the toml description file to configure the project, we often encounter problems with project version declaration and management, such as:

```toml
[package]
name = "libevent_sys"
version = "0.1.0"

[dependencies]
libc = "0.2"

```

Here, the value of the version field in the package section, and the value of the libc field in the dependencies section, and the way these values are written, all involve the issue of semantic version control. Semantic version control uses a set of simple rules and conditions to constrain the configuration and growth of version numbers. These rules are designed according to (but not limited to) conventions that have been widely used by various closed and open source software. Simply put, Semantic Versioning follows these rules:

- Version format: major version number. minor version number. revision number, and the version number increment rules are as follows:

1. Major version number: When you make an incompatible API modification,
2. Minor version number: When you add backward compatible functionality,
3. Revision number: When you made a backward compatible problem fix.

- The previous version number and version compilation information can be added to the end of "major version number. minor version number. revision number" as an extension.

For the specific details of semantic version control, you can refer to [here] (http://semver.org/lang/zh-CN/), I will not repeat them.

# Cargo's toml description file configuration field detailed reference

## [package] paragraph
I don’t want to say much, let’s go straight to the example. Please pay attention to my Chinese explanation in the example. Personally, I think this is more clear:

```toml
[package]
  # Package name, if you need to refer to this package elsewhere, please use this name.
name = "hello_world"

# The current version number, here follows the semver standard, which is the semantic version control standard.
version = "0.1.0" # the current version, obeying semver

# List of all authors of the software
authors = ["you@example.com"]

# A very useful field, if you want to customize your own build workflow,
# Especially when calling external tools to build packages developed in other native languages (C, C++, D, etc.).
# At this time, the custom build process can use the rust language and be written in the "build.rs" file.
build = "build.rs"

# Explicitly declare which files in the package folder are excluded from the project's build process,
# Which files are included in the project's build process
exclude = ["build/**/*.o", "doc/**/*.html"]
include = ["src/**/*", "Cargo.toml"]

# When an error occurs when the package is released to the public repository, enabling this field can prevent this error.
publish = false

# A short introduction about the package.
description = "..."

# The following fields indicate more information about the package repository
documentation = "..."
homepage = "..."
repository = "..."

# As the name suggests, the file pointed to by this field is the legendary ReadMe,
# And, the content of this file will eventually be saved in the registry database.
readme = "..."

# Keywords for classification and retrieval.
keywords = ["...", "..."]

# The license of the package, must be a known standard license listed in the cargo repository.
license = "..."

# The file path corresponding to the non-standard license certificate of the software package.
license-file = "..."
```

## Detailed configuration of dependencies
The most direct way has been discussed in Chapter 5, so I won’t go into details here, for example:

```toml
[dependencies]
hammer = "0.5.0"
color = "> 0.6.0, < 0.8.0"
```

The platform-related dependency definition format remains the same, the difference is that it needs to be defined under the [target] field. For example:

```toml
# Note that the cfg here can use not, any, all and other operators to arbitrarily combine key-value pairs.
# And this usage only supports versions above cargo 0.9.0 (rust 1.8.0).
# If it is a windows platform, this dependency is required.
[target.'cfg(windows)'.dependencies]
winhttp = "0.4.0"

[target.'cfg(unix)'.dependencies]
openssl = "1.0.1"

#If it is a 32-bit platform, this dependency is required.
[target.'cfg(target_pointer_width = "32")'.dependencies]
native = { path = "native/i686" }

[target.'cfg(target_pointer_width = "64")'.dependencies]
native = { path = "native/i686" }

# Another way of writing is to list the full description of the platform
[target.x86_64-pc-windows-gnu.dependencies]
winhttp = "0.4.0"
[target.i686-unknown-linux-gnu.dependencies]
openssl = "1.0.1"

# If using a custom platform, enclose the full path to the custom platform file in double quotes
[target."x86_64/windows.json".dependencies]
winhttp = "0.4.0"
[target."i686/linux.json".dependencies]
openssl = "1.0.1"
native = { path = "native/i686" }
openssl = "1.0.1"
native = { path = "native/x86_64" }

# The format of the [dev-dependencies] paragraph is equivalent to the [dependencies] paragraph,
# The difference is that the dependencies declared in the [dependencies] section are used to build the package,
# The dependencies declared in the [dev-dependencies] paragraph are only used for building tests and performance evaluation.
# In addition, the dependencies declared in the [dev-dependencies] paragraph will not be passed to other projects that depend on this package
[dev-dependencies]
iron = "0.2"

```

## Custom compiler call method template detailed parameters
Cargo has five built-in compiler calling templates, namely dev, release, test, bench, and doc, which are used to define compiler parameters for different types of generated targets. If we want to change these compilation templates, we can define the corresponding fields by ourselves. Values, for example (note: the values listed in the following examples are the system default values corresponding to this template field):

```toml
# Development template, corresponding to `cargo build` command
[profile.dev]
opt-level = 0 # Control the compiler's --opt-level parameter, which is the optimization parameter
debug = true # Control whether the compiler opens the `-g` parameter
rpath = false # Control the `-C rpath` parameter of the compiler
lto = false # Control the `-C lto` parameter, which affects the generation of executable files and static libraries,
debug-assertions = true # Control whether debug assertions are enabled
codegen-units = 1 # Control the compiler's `-C codegen-units` parameter. Note that this field value is ignored when `lto = true`

# release template, corresponding to `cargo build --release` command
[profile. release]
opt-level = 3
debug = false
rpath = false
lto = false
debug-assertions = false
codegen-units = 1

# Test template, corresponding to `cargo test` command
[profile. test]
opt-level = 0
debug = true
rpath = false
lto = false
debug-assertions = true
codegen-units = 1

# Performance evaluation template, corresponding to `cargo bench` command
[profile.bench]
opt-level = 3
debug = false
rpath = false
lto = false
debug-assertions = false
codegen-units = 1

# Documentation template, corresponding to the `cargo doc` command
[profile.doc]
opt-level = 0
debug = true
rpath = false
lto = false
debug-assertions=true
codegen-units = 1

```

It should be noted that when the compiler is invoked, only the template files of the top-level package are valid, and the template definitions of other sub-packages or dependent packages will be overwritten by the templates of the top-level package.

## [features] paragraph
Fields in the [features] section are used for conditional compilation options or optional dependencies. For example:

```toml
[package]
name = "awesome"

[features]
# This field sets the default picklist of optional dependencies,
# Note that "session" here is not a package name,
# But another featrue field session
default = ["jquery", "uglifier", "session"]

# A feature with an empty value like this is generally used for conditional compilation,
# Similar to `#[cfg(feature = "go-faster")]`.
go-faster = []

# This feature depends on the bcrypt package,
# The advantage of this encapsulation is that optional items can be added to the secure-password feature in the future.
secure-password = ["bcrypt"]

# The session field here imports the session field in the feature section of the cookie package
session = ["cookie/session"]

[dependencies]
# Required dependencies
cookie = "1.2.0"
oauth = "1.1.0"
route-recognizer = "=2.1.0"

# optional dependencies
jquery = { version = "1.0.2", optional = true }
uglifier = { version = "1.5.3", optional = true }
bcrypt = { version = "*", optional = true }
civet = { version = "*", optional = true }
```

If other software packages depend on the above awesome package, you can write this in their description file:

```toml
[dependencies.awesome]
version = "1.3.5"
default-features = false # Disable the default features of awesome
features = ["secure-password", "civet"] # Use the features listed here
```

The following rules need to be followed when using features:

- The feature name cannot conflict with the package name that appears in this description file
- Except the default feature, all other features are optional
- features cannot be included in a loop with each other
- Development dependencies cannot be included
- The features group can only depend on optional packages

An important use of features is that when developers need to release the final software package, they can declare the features exposed to end users when building, which can be achieved by the following command:

```
$ cargo build --release --features "shumway pdf"
```

## About testing
When running the cargo test command, cargo will do the following:

- compile and run the unit tests in the package source marked with #[cfg(test)]
- Compile and run documentation tests
- Compile and run integration tests
- compile examples

## Configure the build target
All fields such as [[bin]], [lib], [[bench]], [[test]] and [[example]] provide similar configuration for how the build target should be built. For example (the values of each field in the [lib] section in the following example are default values):

```toml
[lib]
# The library name, the default is the same as the project name
name = "foo"

# This option is only used in the [lib] section, which determines how the build target is built,
# Can take one of the three values of dylib, rlib, staticlib, which means generating dynamic library, r library or static library.
crate-type = ["dylib"]

# The path field declares the relative path of this build target relative to the cargo.toml file
path = "src/lib.rs"

# Unit test switch options
test=true

# document test switch options
doctest = true

# Performance evaluation switch options
bench = true

# Document generation switch options
doc = true

# Whether to build as a compiler plugin switch option
plugin = false

# If set to false, `cargo test` will ignore the --test argument passed to rustc.
harness = true
```
