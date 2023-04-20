
/// We can use clap in two different ways
/// 1. Declaratively, by `declarative macros` applied to our structs.
/// 2. Programmatically, by using its `builder like` api in the code.

/// Method 1
struct Options1 {

}

use std::process;
use clap::{Arg, ArgMatches, App, SubCommand};

fn main() {
    let matches = App::new("24daysofrust")
        .version("0.1")
        .author("Zbigniew Siciarz")
        .about("learn you some Rust!")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("verbosity level"))
        .get_matches();
    if let Err(e) = run(matches) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}