

// Note: there are many options available with std::env module. Please look at documents.
fn main() {
    println!("{}", env!("SHELL"));


    for item in std::env::vars() {
        println!("{}={}", item.0, item.1)
    }
}