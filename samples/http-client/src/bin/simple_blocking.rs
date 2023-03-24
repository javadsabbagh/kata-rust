#![allow(unused_imports)]

use std::collections::{BTreeMap, HashMap};
use std::iter::Map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<BTreeMap<String, String>>()?;  // Should be key-value data structure, i.e. any Map

    println!("{:#?}", resp); // # is for formatting (JSON Beautify)
    Ok(())
}