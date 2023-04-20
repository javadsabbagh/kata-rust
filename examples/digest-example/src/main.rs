use crypto::common::typenum::Sha256;

fn main() {
    let input = "Hello world!";
    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
}