use regex::Regex;

fn main() {
    // raw strings make working with regex easy, i.e. no need to skip many special characters.
    let numbers = Regex::new(r"(\+/-)?\d+").unwrap();
    // let numbers = Regex::new(r#"^(\+/-)?\d+$"#).unwrap(); it fails following assert!

    // use is_match method to check if a given string argument matches the regex or not
    assert!(numbers.is_match("Number is in text but it also contains non-number words: 22323 "));

}
