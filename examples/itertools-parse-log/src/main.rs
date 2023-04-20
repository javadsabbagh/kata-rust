
mod file;
mod log;

use std::fs::read;
use file::read_lines;
fn main() {
    let log = read_lines("sample-apache.log").unwrap();
}
