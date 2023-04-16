# linked list

## Introduction to Linked List
A linked list is a non-sequential and non-sequential storage structure on a physical storage unit, and the logical order of data elements is realized through the link order of pointers in the linked list. The linked list is composed of a series of nodes (each element in the linked list is called a node), and the nodes can be dynamically generated at runtime. Each node consists of two parts: one is a data field that stores data elements, and the other is a pointer field that stores the address of the next node. Since it does not have to be stored in order, the linked list can reach the complexity of O(1) when inserting at a given position, which is much faster than another linear list sequence table, but it is difficult to find a node in the ordered data or access a specific The target node requires O(n) time, and the corresponding time complexities of the linear table are O(logn) and O(1) respectively.

>Using the linked list structure can overcome the disadvantage that the array needs to know the data size in advance. The linked list structure can make full use of the computer memory space and realize flexible memory dynamic management. However, the linked list loses the advantage of random read of the array, and at the same time, the linked list has a relatively large space overhead due to the increase of the pointer field of the node. The most obvious advantage of the linked list is that the way the conventional array arranges the associated items may be different from the order of these data items in memory or on the disk, and the access of data often needs to be converted in different arrangement orders. Linked lists allow insertion and removal of nodes at arbitrary positions on the list, but do not allow random access. There are many different types of linked lists: singly linked lists, doubly linked lists, and circular linked lists.

Let's see how we implement the linked list step by step:

## Define the linked list structure

```rust
use List::*;

enum List {
    // Cons: a tuple structure containing one element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: Indicates the end of a linked list node
    Nil,
}
```

## How to implement linked list

```rust
implList {
    // Create an empty linked list
    fn new() -> List {
        // `Nil` is of type `List`. Because earlier we used `use List::*;`
        // So no need to use List::Nil like this
        Nil
    }

    // Add an element node in front, and link the old linked list and return the new linked list
    fn prepend(self, elem: u32) -> List {
        // `Cons` is also of type List
        Cons(elem, Box::new(self))
    }

    // return the length of the linked list
    fn len(&self) -> u32 {
        // `self` is of type `&List`, `*self` is of type `List`,
        // Matching a type `T` is better than matching a reference `&T`
        match *self {
            // Since `self` is borrowed, ownership of tail cannot be transferred
            // so use tail's reference
            Cons(_, ref tail) => 1 + tail.len(),
            // Basic rule: so the length of the empty linked list is 0
            Nil => 0
        }
    }

    // return the string representation of the linked list
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`
                // but returns a string on the heap instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}
```

## code test

```rust
fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```

## practise

Implement a two-way circular linked list based on the above code.

>Doubly linked list is also called double linked list, which is a kind of linked list. Each data node in it has two pointers, which point to the direct successor and direct predecessor respectively. Therefore, starting from any node in the doubly linked list, you can easily access its predecessor node and successor node. Generally, we construct a two-way circular linked list.
>Circular linked list is another form of linked storage structure. Its characteristic is that the pointer field of the last node in the list points to the head node, and the entire linked list forms a ring.
