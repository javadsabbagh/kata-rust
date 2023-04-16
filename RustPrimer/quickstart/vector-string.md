# arrays, dynamic arrays and strings
## Arrays and dynamic arrays
### array array
Rust uses arrays to store the same type of dataset.
`[T; N]` means an array with type T and N elements. The size of the array is fixed.

**example:**

```rust
fn main() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    for x in &array {
        println!("{} ", x);
    }
}
```

### Dynamic Array Vec
Dynamic array is a continuous dynamic data type based on heap memory application. It has index, push and pop with O(1) time complexity.

**example:**

```rust
//Create an empty Vec
let v: Vec<i32> = Vec::new();
//Create an empty Vec using a macro
let v: Vec<i32> = vec![];
//Create a Vec with 5 elements
let v = vec![1, 2, 3, 4, 5];
// create ten zeros
let v = vec![0; 10];
//Create a variable Vec and push into element 3
let mut v = vec![1, 2];
v. push(3);
//Create a Vec with two elements and pop an element
let mut v = vec![1, 2];
let two = v. pop();
//Create a mutable Vec with three elements, and index a value and modify a value
let mut v = vec![1, 2, 3];
let three = v[2];
v[1] = v[1] + 5;
```

## string
There are two string types in Rust. `String` and `str`.

### &str
The `str` type is basically not used very much, and the `&str` type is usually used, which is actually the slice form `&[u8]` of the `[u8]` type. This is a fixed size string type.
A common string literal is the `&'static str` type. This is a &str type with `'static` lifetime.

**example:**

```rust
// string literal
let hello = "Hello, world!";

// with explicit type flags
let hello: &'static str = "Hello, world!";
```

###String
`String` is a structure with `vec:Vec<u8>` member, you can understand it as a dynamic form of `str` type.
Their relationship is equivalent to that of `[T]` and `Vec<T>`.
Apparently the `String` type also has pushes and pops.

**example:**

```rust
// create an empty string
let mut s = String::new();
// Convert from `&str` type to `String` type
let mut hello = String::from("Hello, ");
// Push characters and push string slices
hello.push('w');
hello.push_str("orld!");

// Pop the character.
let mut s = String::from("foo");
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('f'));
assert_eq!(s.pop(), None);
```
