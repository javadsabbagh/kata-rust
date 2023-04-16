# queue

## Queue Profile
Queue is a special linear table, which is special in that it only allows deletion at the front end (front) of the table, and insert operation at the back end (rear) of the table. Like a stack, a queue is an operation subject to A linear list of constraints. The end of the insertion operation is called the tail of the queue, and the end of the deletion operation is called the head of the queue. When there are no elements in the queue, it is called an empty queue.

>In the formation process of the queue, the principle of linear linked list can be used to generate a queue. The queue based on the linked list is less efficient to dynamically create and delete nodes, but it can grow dynamically. The queue uses **FIFO(first in first out)**, new elements (elements waiting to enter the queue) are always inserted at the end of the linked list, and when reading, they are always read from the head of the linked list. Each time an element is read, an element is freed. The so-called dynamic creation, dynamic release. Therefore, there is no problem such as overflow. Since the linked list is indirectly formed by the structure, traversal is also convenient.

## Queue implementation
Let's take a look at our simple Queue implemented using Vec:

Main implementation of `new( ), push( ), pop( )` three methods

```rust
#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    fn new() -> Self {
        Queue{qdata: Vec::new()}
    }

    fn push(&mut self, item:T) {
        self.qdata.push(item);
    }

    fn pop(&mut self) -> T{
        self.qdata.remove(0)
    }
}

fn main() {
    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);
    q.pop();
}
```

## practise
It looks much simpler than the Stack we implemented in the previous section. But this Queue implementation has bugs.

Exercise: Find a bug in this code and fix it.

Tip: The `pop( )` method has a bug, please refer to the implementation in the Stack section, and use Option to handle it.
