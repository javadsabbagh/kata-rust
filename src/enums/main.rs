// Enum by value

#[derive(Debug)]
enum Coin {
    Penny = 1,
    Nickle = 5,
    Dime = 10,
    Quarter = 25,
    //Anothr_ITEM // it would be 26 if no value was implicitly set
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

    use NodeType::*;
    let node = KVM(VM {
        name: "Ubuntu 22.04 LTS Server".to_string(),
        ip: String::from("10.0.0.5")    
    });  
    
    // Note: three ways to convert string literal into String object: "abc".as_string(), String::from("abc"), and format!("{}", "abc") macro 


    println!("{:?}", node); // Note: when a type implements Debug trait, all other integrating parts also must impl Debug.
}
