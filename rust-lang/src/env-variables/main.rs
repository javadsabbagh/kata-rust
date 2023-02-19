

// Note: there are many options available with std::env module. Please look at documents.
fn main() {

    println!("{:?}", std::env::current_exe());  // returns io::Result<PathBuf>

    // iterate through all environment variables, each item is a tuple of key-value.
    for item in std::env::vars() {
        println!("{}={}", item.0, item.1)
    }

    // using vec! macro, collects enviroment variable at compile time
    println!("{}", env!("SHELL"));            // compile error if vaiable does not exist.
    println!("{:?}", option_env!("SHELL"));   // returns Option and no compile error if variable does not exists.
}