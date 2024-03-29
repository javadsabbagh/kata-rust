## Create random passwords from a set of alphanumeric characters

Randomly generates a string of given length ASCII characters in the range `A-Z,
a-z, 0-9`, with [`Alphanumeric`] sample.

```rust
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}
```

[`Alphanumeric`]: https://docs.rs/rand/*/rand/distributions/struct.Alphanumeric.html
