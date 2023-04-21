## Check number of logical cpu cores

Shows the number of logical CPU cores in current machine using [`num_cpus::get`].

```rust
fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
```
