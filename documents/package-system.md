
## Packages in Rust

There are two kinds of packages in Rust:
 - Library
 - Application

### Library
Each crate can have at most one library, but several binary applications.

There is a convention that library root is located in **src/lib.rs**, however you can use other name (for any good reason)
in that case you should use lib section in Cargo.toml file:

```toml
[lib]
path = "src/lib_custom_name.rs"
```

## Application
Each crate can have any number of runnable applications. The default one is located at **src/main.rs**,
and other binaries by convention are placed at **src/bin/** folder. By the way you can always override and customize application path
in bin section of Cargo.toml file:

```toml
[[bin]]
name = "custom-app-name"
path = "src/app-path/app-name.rs"
```

There can be several of [[bin]] sections in project config file.

### Terminology
- package: Is a container of project crates which contains project config file needed to building and running.
- crate: A crate is in source or binary (artifact) form. Source crate contains source files of a project.
- library: A library crate with no executable programs. Used for code sharing in binary format.
- application: Binary and executable program.
- module: Used for organizing code into different namespaces, guarantees scope and privacy of code.

Confusion other with programming languages, e.g. Java:
- Package: Similar to **maven artifact** in source or binary form. In java pom.xml is similar to Cargo.toml file.
- Crate: Comparing to Java, a crate is equivalent to **maven modules**, in source java files or compiled binary, i.e. JAR files. 
- Library/Application: In java there is no difference between library and executable source/binary files.
- Modules: In java they are called **packages** with strict directory name conventions!