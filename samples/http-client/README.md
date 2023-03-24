
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