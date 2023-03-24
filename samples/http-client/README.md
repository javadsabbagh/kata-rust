
# reqwest
Sample programs in this project relies on `reqwest` project.
Please, consider reading the original source in its [GitHub](https://github.com/seanmonstar/reqwest) repository.

### Non-Blocking
reqwest is non-blocking by default, and needs an asynchronous framework (e.g. tokio) to run.

### Blocking
For blocking service calls enable `blocking` feature in Cargo.toml, then use `reqwest::blocking` associated method.

### TLS
For using SSL http protected resources, enable `rustls-tls` feature. `rust-tls` is a pure rust-native implementation of TLS, instead
of using OpenSSL (implemented in C) in rust projects.

### Library Tips
1. For blocking usage after every method call use `?` operator to unwrap the `Result`:
```rust
.get()?
.json()?
```
2. For non-blocking usage use `await?` after every method call to wait for the `Future` and unwrap the `Result`:
```rust
.get()
.await?
.json()
.await?
```
3. Use `json()` method call with turbo fish operator, instead of compiler type inference. It's more rust-idiomatic:
```rust
.json::<HashMap<String, String>>()
```
i.e.
Do this:
```rust
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
```

Don't do this:
```rust
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")
        .await?
        .json()
        .await?;
```