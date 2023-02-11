

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
struct VM {}

#[derive(Debug)]
struct Container {}

#[derive(Debug)]
enum NodeType {
    KVM(VM),
    Docker(Container)
}

fn main() {
    use Coin::*;

    let coin = Penny;
    println!("{:?}", coin);   // Note: unless Debug directive is not set it gives an error that Debug trait is not implemented for Coin enum


    use NodeType::*;
    let node = KVM(VM{});

    println!("{:?}", node); // Note: when a type implements Debug trait, all other integrating parts also must impl Debug.
}