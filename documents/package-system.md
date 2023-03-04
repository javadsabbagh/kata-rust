
## Packages in Rust

There are two kinds of packages in Rust:
 - Library
 - Application


### Library
Each crate can have at most one library, but several binary applications.

There is a convention that library root is located in "src/lib.rs", however you can use other name (for any good reason)
in that case you should use lib section in Cargo.toml file:

```toml
[lib]
path = "src/lib_custom_name.rs"
```