// Enum by value

#[derive(Debug)]
enum Coin {
    Penny = 1,
    Nickle = 5,
    Dime = 10,
    Quarter = 25,
    //Anothr_ITEM // it would be 26 if no value was implicitly set
}

trait Money {
    fn format(&self) -> String;
    fn to_dollar(&self) -> f32;
}

use Coin::*;

impl Money for Coin {
    fn format(&self) -> String {
        match self {
            Penny => format!("{}¢", Penny as u8),
            Nickle => format!("{}¢", Nickle as u8),
            Dime => format!("{}¢", Dime as u8),
            Quarter => format!("{}¢", Quarter as u8),
        }
    }

    fn to_dollar(&self) -> f32 {

        // Just for casting demonhstration:
        // enum value is set to integer so it cannot be cast float directly!!
        // but, it can be casted to any other iteger type
        match self {
            Penny =>  Penny as u8 as f32 / 100.0,
            Nickle => Nickle as i32 as f32 / 100.0,
            Dime => Dime as u64 as f32 / 100 as f32,
            Quarter => Quarter as u16 as f32 / 100 as f32,
        }
    }
}

// Enum by variable

#[derive(Debug)]
struct VM {
    name: String,
    ip: String,
}

#[derive(Debug)]
struct Container {}

#[derive(Debug)]
enum NodeType {
    KVM(VM),
    Docker(Container),
}

fn main() {
    use Coin::*;

    let coin = Penny;
    println!("{:?}", coin); // Note: unless Debug directive is not set it gives an error that Debug trait is not implemented for Coin enum

    println!("This coin ({}) eqauls to {} dollar", coin.format(), coin.to_dollar());

    use NodeType::*;
    let node = KVM(VM {
        name: "Ubuntu 22.04 LTS Server".to_string(),
        ip: String::from("10.0.0.5"),
    });

    // Note: three ways to convert string literal into String object: "abc".as_string(), String::from("abc"), and format!("{}", "abc") macro

    println!("{:?}", node); // Note: when a type implements Debug trait, all other integrating parts also must impl Debug.
}
