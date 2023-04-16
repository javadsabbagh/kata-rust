# References & Borrowing (References&Borrowing)


As shown above, Owership makes it "complex" for us to change the value of a variable. Can we change the value of a variable at will like other programming languages? The answer is yes.

The ownership system allows us to do this through "Borrowing". This mechanism is very similar to the "read-write lock" in other programming languages, that is, at the same time, only one "write lock" or multiple "read locks" can be owned, and "write lock" and "read lock" are not allowed. appear at the same time. Of course, this is also a typical way to ensure consistency during data reading and writing. It's just that Rust completes this (Borrowing) check during compilation, not at runtime, which is why other language programs are prone to deadlock or wild pointer problems during runtime.


Borrowing is done with the **&** symbol:

```rust
fn main() {
	let x: Vec<i32> = vec!(1i32, 2, 3);
	let y = &x;
	println!("x={:?}, y={:?}", x, y);
}
```

Borrowing(**&x**) does not cause ownership moved, so println can access both x and y at the same time.
By reference, you can complete the modification of ordinary types.

```rust
fn main() {
	let mut x: i32 = 100;
	{
		let y: &mut i32 = &mut x;
		*y += 2;
	}
	println!("{}", x);
}
```

### The difference between borrowing and reference

Borrowing and citation are a complementary relationship. If B is a reference to A, it can also be said that B borrowed A.

Pretty close, right, but the word to borrow means to return. So when using references in Rust, be sure to pay attention to where and when to "return" the borrow/reference correctly.
The "Advanced" section at the end will give detailed examples.

###rule

1. In the same scope, there is at most one mutable borrow (&mut T) for specific data, or 2.
2. In the same scope, specific data can have 0 or more immutable borrows (&T), but cannot have any mutable borrows.
3. The borrow is released after going out of scope.
4. The source variable cannot be accessed until the mutable borrow is released.

### Mutability
Borrowing is also divided into "immutable borrowing" (default, **&T**) and "variable borrowing" (**&mut T**).

As the name suggests, "immutable borrows" are read-only and cannot update the referenced content.

```rust
fn main() {
	let x: Vec<i32> = vec!(1i32, 2, 3);

	// There can be multiple immutable borrows at the same time
	let y = &x;
	let z = &x;
	let m = &x;

	//ok
	println!("{:?}, {:?}, {:?}, {:?}", x, y, z, m);
}
```

Again, there can only be one mutable borrow (&mut T) in the same scope, and the borrowed variable itself must be mutable:

```rust
fn main() {
  // source variable x variability
   let mut x: Vec<i32> = vec!(1i32, 2, 3);

   // There can only be one mutable borrow
   let y = &mut x;
   // let z = &mut x; // error
     y.push(100);

   //ok
   println!("{:?}", y);

  //Error, mutable borrow not released, source variable not accessible
  // println!("{:?}", x);
} //y is destroyed here
```

### Advanced Example
The following complex example has detailed annotations. It doesn’t matter if you don’t understand it. You can think about this example carefully after completing the study of Lifetimes (life cycle).

```rust
fn main() {
    let mut x: Vec<i32> = vec!(1i32, 2, 3);

    //Update the array
    //The array is variable borrowed in push, and the borrow is destroyed when the push function exits
     x. push(10);

     {
        // variable borrow 1
        let mut y = &mut x;
         y.push(100);

         //Variable borrowing 2, note: here is a borrowing of y, and no more borrowing of x,
         //Because y is still alive at this time.
         let z = &mut y;
         z.push(1000);

        println!("{:?}", z); //print: [1, 2, 3, 10, 100, 1000]
     } //y and z are destroyed here, and the borrow is released.


    //Access x is normal
    println!("{:?}", x); //print: [1, 2, 3, 10, 100, 1000]
}
```

####Summarize
1. Borrowing does not change the owner of the memory (Owner), borrowing is only a temporary reference to the source memory.
2. During the borrow period, the borrower can read and write the memory, and the owner is prohibited from reading and writing the memory; and the owner guarantees that the memory will not be released or transferred if there is a "borrow".
3. Variables that lose ownership cannot be borrowed (accessed).
4. During the lease period, the memory owner guarantees that the memory will not be released/transferred/mutably leased, but if it is in the case of **non-mutable lease**, the owner is allowed to continue **non-mutable It was rented out.
5. After the borrow period expires, the owner withdraws the read and write permissions
6. The borrow period is shorter than the lifetime of the borrowee (owner).

> Remarks:
> Borrowing cycle refers to the effective time period of borrowing.
