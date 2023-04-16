# Hash table HashMap

Like the dynamic array `Vec`, the hash table (HashMap) is also one of Rust's built-in collection types, and it belongs to the `std::collections` module.

It provides a query method with an average complexity of `O(1)`, which is one of the necessary types for fast search.

Here, I will mainly introduce several typical usages of HashMap.

## HashMap requirements

As the name suggests, HashMap requires a Key type that can be hashed (implements the Hash trait), and a Value type whose size is known at compile time.
At the same time, Rust also requires that your Key type must be comparable. In Rust, you can easily add compiler attributes to your type:

```rust
#[derive(PartialEq, Eq, Hash)]
```

In this way, your type can be converted into a type that can be used as a Hash Key.
However, if you want to implement the `Hash` trait yourself, you need to keep two things in mind:

* 1. If Key1==Key2, then there must be Hash(Key1) == Hash(Key2)
* 2. Your Hash function itself cannot change your Key value, otherwise it will cause a logic error (difficult to troubleshoot, the kind that is over when you encounter it)

What? You see that the function in `std::hash::Hash` does not have `&mut self`! However, don't forget that there are `Cell` and `RefCell` in Rust, they provide the internal mutability of immutable objects, how to change, please refer to Chapter 20.

In addition, make sure that the Hash function you write will not be easily collided, that is, `Key1! = Key2`, but `Hash(Key1)==Hash(Key2)`, the collision is serious, and HashMap may even degenerate into Linked list!

Here the author suggests, don't bother, just follow the simplest way.

## CRUD

For this practical type, we recommend an example to explain:

```rust
use std::collections::HashMap;

// statement
let mut come_from = HashMap::new();
// insert
come_from.insert("WaySLOG", "HeBei");
come_from.insert("Marisa", "U.S.");
come_from.insert("Mike", "HuoGuo");

// look up the key
if !come_from.contains_key("elton") {
    println!("Oh, we found {} people, but poor Elton cat is still homeless", come_from.len());
}

// Delete element according to key
come_from.remove("Mike");
println!("Mike cat's hometown is not hot pot! Not hot pot! Not hot pot! It's delicious!");

// Use the return of get to determine whether the element exists
let who = ["MoGu", "Marisa"];
for person in &who {
    match come_from.get(person) {
        Some(location) => println!("{} from: {}", person, location),
        None => println!("{} is also homeless.", person),
    }
}

// loop through the output
println!("So, everyone?");
for (name, location) in &come_from {
    println!("{} from: {}", name, location);
}
```

This code outputs:

```
Oh, we tracked down 3 people, but poor Elton the cat is still homeless
Mike's hometown is not hot pot! Not hot pot! Not hot pot! Delicious though!
MoGu is also homeless.
Marisa From: U.S.
So, what about everyone?
MarisaFrom: U.S.
WaySLOGFrom: HeBei
```

## entry

In the process of programming, we often encounter such a scenario, counting how many times all the characters in a string appear in total. We can always do this with the help of the built-in Map type in various languages, but almost all of them are not satisfactory. What many people hate is: why do I have to write a big if condition to judge whether this character appears in the dictionary! Are you bothered? bother! As a result, modern programming languages have begun to integrate features (methods) similar to `setdefault` in Python. The following is a piece of Python code:

```python
val = {}
for c in "abcdefasdasdawe":
    val[c] = 1 + val.setdefault(c, 0)
print val
```

Well, it always feels weird. So how does Rust solve this problem?
The following is excerpted from annotation library api annotations:

```rust
use std::collections::HashMap;

let mut letters = HashMap::new();

for ch in "a short treatise on fungi".chars() {
    let counter = letters.entry(ch).or_insert(0);
    *counter += 1;
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters.get(&'y'), None);
```

Rust provides us with an api called `entry`, which is very interesting. Compared with Python, we don't need to visit the original map twice during an iteration, we only need to borrow the Entry type from the entry (this type holds The original HashMap reference) can modify the original data. In terms of syntax, there is no doubt that Rust is more intuitive and specific in this regard.
