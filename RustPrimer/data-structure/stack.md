# stack

## Introduction to the stack

- As a data structure, a stack is a special linear table that can only perform **insert** and **delete** operations on **one end**.

- It stores data according to the principle of **first in, last out**, the data that enters first is pushed into the bottom of the stack, and the last data is on the top of the stack. read out).

>Stack (stack), also known as stack, is a linear table with limited operations. The limitation is that insertion and deletion operations are only allowed at one end of the table. This end is called the top of the stack, and the other end is called the bottom of the stack. Inserting a new element into a stack is also called pushing, pushing, or pushing. It is to put the new element on top of the top element of the stack to make it a new top element of the stack; deleting an element from a stack is also called stacking or stacking. Unstack, it is to delete the top element of the stack, so that the adjacent element becomes the new top element of the stack.

## Implementation steps of the stack:

- Define a stack structure `Stack`
- Define the stack point `StackNode` that makes up the stack structure
- Implement stack initialization function `new( )`
- Implement push function `push( )`
- Implement unstack function `pop( )`

## Define a stack structure `Stack`

```rust
#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}
```

Let's analyze step by step

- The `#[derive(Debug)]` in the first line is to make the `Stack` structure printable for debugging.
- The second line defines a `Stack` structure, which contains a generic parameter `T`.
- The third line is more complicated, introduced when defining `StackNode`

## Define the stack point `StackNode` that makes up the stack structure

```rust
#[derive(Clone,Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}
```

In the third line of this code, we define a `val` to hold the value of `StackNode`.

> Now let's focus on the fourth line:
Let's take a look **from the inside out**, first of all `Box<StackNode<T>`, where `Box` is the type used by Rust to explicitly allocate heap memory:

> `pub struct Box<T> where T: ?Sized(_);`
[For detailed documentation, please refer to Rust's standard library](http://doc.rust-lang.org/nightly/std/boxed/struct.Box.html)

> A unified abstraction in Rust with a powerful type system. Here it is equivalent to applying for a piece of memory in the heap space to save `StackNode<T>`.

> **Why did you do this? What happens if you don't use Box packaging? **

> If you don’t use Box encapsulation, the rustc compiler will report an error. In Rust, rustc uses stack space by default, but here `StackNode` is defined using a recursive data structure, and the type of the next attribute is `StackNode<T>`, And this type cannot be determined in size, and all types of this indeterminate size cannot be stored in the stack space. So you need to use `Box` to encapsulate. In this case, the type of `next` is a pointer to a certain piece of heap space, and the size of the pointer can be determined, so it can be stored in the stack space.

> **So why do you need to use `Option` to encapsulate it? **

> `Option` is an abstract type in Rust, defined as follows:
>

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Option includes elements, None and Some(T), so that it is easy to describe that when next points to the element at the end of the stack, it is all under the Option type, which is convenient for function realization and error handling. Option also has many powerful functions, readers can refer to the following links:

[Option standard library documentation](http://doc.rust-lang.org/nightly/std/option/enum.Option.html)

[Error Handling in Rust](http://blog.burntsushi.net/rust-error-handling/)

[Error handling for rustbyexample](https://doc.rust-lang.org/stable/rust-by-example/error.html)

## Implement `new( ) push( ) pop( )`
Next is to implement the main functions of the stack.

```rust
impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{ top: None }
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            },
        }
    }
}
```

- `new( )` is relatively simple. When the Stack is initialized, it is empty, and the top element of the stack `top` has no value, so `top` is `None`.

- The main function of `push( )` is to push elements into the stack, point the new StackNode to the old value in the Stack, and update the top of the Stack to point to the new value.
> One thing to note here is that in line 8 of the code, `let next = self.top.take();` uses the take method of the Option type:
`fn take(&mut self) -> Option<T>`
It will take the value of type Option and change its elements to None

- The function of `pop( )` is to take out the element at the top of the stack, and return None if the top of the stack is None.

## Complete code (including simple tests)

```rust
#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Clone,Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl <T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode { val: val, next: None }
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{ top: None }
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            },
        }
    }
}

fn main() {
    #[derive(PartialEq,Eq,Debug)]
    struct TestStruct {
        a: i32,
    }

    let a = TestStruct{ a: 5 };
    let b = TestStruct{ a: 9 };

    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}
```
