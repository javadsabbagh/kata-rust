
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