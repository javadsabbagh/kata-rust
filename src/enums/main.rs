

#[derive(Debug)]
enum Coin {
    Penny = 1,
    Nickle = 5,
    Dime = 10,
    Quarter = 25
}

fn main() {
    use Coin::*;

    let coin = Penny;
    println!("{:?}", coin);   // unless Debug directive is not set it gives an error that Debug trait is not implemented for Coin enum
}