**Ownership**
-------------
Before entering the topic, let's recall the general programming language knowledge.
For general programming languages, it is common to declare a variable first and then initialize it.
For example in C language:
```c
int* foo() {
     int a; // scope of variable a begins
     a = 100;
     char *c = "xyz"; // scope of variable c begins
     return &a;
} // The scope of variables a and c ends
```

Although it can be compiled, this is a very bad piece of code. In reality, I believe that everyone will not write it like this. The variables a and c are both local variables. After the function ends, the address of the local variable a will be returned, but the local variable `a` exists on the stack. After leaving the scope, the memory on the stack requested by the local variable will be reclaimed by the system, thus Caused problems with `Dangling Pointer`. ** This is a very typical memory safety problem. Memory safety issues like this exist in many programming languages**. Let’s look at the variable `c` again. The value of `c` is a constant string, which is stored in the constant area. Maybe we only call this function once, and we may no longer want to use this string, but `xyz` is only available when the entire program ends. Only after the system can reclaim this piece of memory, does this make programmers feel helpless?
> Remarks: For `xyz`, you can manually manage (apply and release) memory through the heap according to the actual situation.

Therefore, memory safety and memory management are usually two major headaches in the eyes of programmers. What's exciting is that Rust no longer makes you worry about memory safety or the trouble of memory management. How does Rust do this? Please read below.

### **Binding**
**Important**: First of all, it must be emphasized that there is no concept of variable in Rust, but it should be called `identifier`, and the target `resource` (memory, storing value) `bind` to this `identifier symbol `:
```rust
{
     let x: i32; // identifier x, not bound to any resource
     let y: i32 = 100; // identifier y, bound resource 100
}
```

Ok, let's continue to look at the following piece of Rust code:
```rust
{
     let a: i32;
     println!("{}", a);
}
```
The above defines an i32 type identifier `a`, if you directly `println!`, you will receive an error report:
> error: use of possibly uninitialized variable: `a`

This is **Because Rust does not initialize variables by default like other languages, Rust clearly stipulates that the initial value of a variable must be determined by the programmer**.

Correct way:
```rust
{
     let a: i32;
     a = 100; //must initialize a
     println!("{}", a);
}
```
In fact, the **`let`** keyword does not just mean declaring variables, it also has a special and important concept-**binding**. In layman's terms, the `let` keyword can "bind" an identifier to a section of memory area. After binding, this section of memory is owned by this identifier, and this identifier also becomes the only one of this section of memory* *owner**.
Therefore, `a = 100` has several actions. First, allocate an `i32` resource on the stack memory and fill it with the value `100`. Then, bind this resource to `a`, so that `a `Become the owner of the resource.

### **Scope**
Like C, Rust defines scope through `{}` braces:
```rust
{
     {
         let a: i32 = 100;
     }
     println!("{}", a);
}
```
After compiling, you will get the following `error` error:
>b.rs:3:20: 3:21 error: unresolved name `a` [E0425]
b.rs:3 println!("{}", a);

Like the C language, after the local variable leaves the scope, the variable will be destroyed immediately; **But the difference is that Rust will be destroyed and released together with the owner variable along with the memory bound to the variable, whether it is a constant string or not* *. So in the above example, if a is destroyed and accessed again, it will prompt an error that the variable `a` cannot be found. All of this is done during compilation.

### **Move Semantics (move)**
First look at the following code:
```rust
{
     let a: String = String::from("xyz");
     let b = a;
     println!("{}", a);
}
```
After compiling, you will get the following error:
> c.rs:4:20: 4:21 error: use of moved value: `a` [E0382]
c.rs:4 println!("{}", a);

The error means that the variable `a` that was `moved` was accessed in `println`. Then why is there such an error? What is the specific meaning?
In Rust, another mechanism that complements the concept of "binding" is "move ownership", which means, **you can transfer the ownership of resources (ownership) from one binding to another binding** , this operation is also done through the `let` keyword. Unlike binding, the lvalue and rvalue on both sides of `=` are two identifiers:
```rust
grammar:
     let identifier A = identifier B; // transfer ownership of resource bound by "B" to "A"
```
The memory before and after move is shown as follows:
> **Before move:**
a <=> memory (address: **A**, content: "xyz")
**After move:**
a
b <=> memory (address: **A**, content: "xyz")

The moved variable cannot continue to be used. Otherwise, an error `error: use of moved value` will be displayed.

Some people here may wonder, after the move, if variable A and variable B leave the scope, will the corresponding memory cause a "Double Free" problem? The answer is no, **Rust stipulates that only after the owner of the resource is destroyed can the memory be released, and no matter whether the resource has been `move` multiple times, there is only one `owner` at the same time, so the memory of the resource will only be moved `free` once**.
Through this mechanism, memory safety is guaranteed. Do you feel powerful?


### **Copy Features**
Some readers wrote the following example following the example in the "move" section, and then said "a is accessible after being moved":
```rust
     let a: i32 = 100;
     let b = a;
     println!("{}", a);
```
Compilation does pass, outputting `100`. Why is this, is it contrary to the conclusion in the move section?
In fact, this is not the case, it actually depends on whether the variable type implements the `Copy` feature. For variables that implement the `Copy` feature, the resource will be copied to the new memory area during move, and the resource `binding` of the new memory area will be `b`.
> **Before move:**
a <=> memory (address: **A**, content: 100)
**After move:**
a <=> memory (address: **A**, content: 100)
b <=> memory (address: **B**, content: 100)

`a` and `b` before and after the move correspond to different resource memory addresses.

In Rust, the basic data types (Primitive Types) all implement the Copy feature, including i8, i16, i32, i64, usize, u8, u16, u32, u64, f32, f64, (), bool, char and so on. For other data types that support Copy, please refer to the [Copy Chapter](https://doc.rust-lang.org/std/marker/trait.Copy.html "Copy Trait") of the official document.

### **Shallow copy and deep copy**
The difference between the usage of move String and i32 in the previous example is actually similar to the difference between "shallow copy" and "deep copy" in many object-oriented programming languages. For basic data types, "deep copy" and "shallow copy" have the same effect. For reference object types, "shallow copy" is more like just copying the memory address of the object.
What if we want to implement a "deep copy" of `String`? You can directly call the Clone feature of `String` to copy the value of the memory instead of simply copying the address.
```rust
{
     let a: String = String::from("xyz");
     let b = a.clone(); // <- note the clone here
     println!("{}", a);
}
```
At this time, it can be compiled and successfully printed "xyz".

The effect after clone is equivalent to the following:
> **Before move:**
a <=> memory (address: **A**, content: "xyz")
**After move:**
a <=> memory (address: **A**, content: "xyz")
b <=> memory (address: **B**, content: "xyz")
Note that the resource values corresponding to a and b are the same, but the memory addresses are different.


### **Variability**
Through the above, we have already learned about variable declaration, value binding, and move semantics, etc., but we have not performed such a simple operation as modifying variable values, which seems so simple that it is not worth mentioning in other languages. But things are hidden in Rust.
Thinking like other programming languages, modify the value of a variable:
```rust
let a: i32 = 100;
a = 200;
```
Sorry, such a simple operation still reports an error:
> error: re-assignment of immutable variable `a` [E0384]
<anon>:3 a = 200;

Cannot assign to **immutable binding**. If you want to modify the value, you must declare the binding as mutable with the keyword mut:
```rust
let mut a: i32 = 100; // declare that a is mutable through the keyword mut
a = 200;
```

**Thinking of "immutable", we immediately thought of `const` constants, but immutable bindings and `const` constants are two completely different concepts; first of all, "immutable" should be called "immutable" accurately Binding" is used to constrain the binding behavior. After "immutable binding", the resource content cannot be changed by the original "owner". **

**For example:**
```rust
let a = vec![1, 2, 3]; //immutable binding, a <=> memory area A(1,2,3)
let mut a = a; //variable binding, a <=> memory area A(1,2,3), note that a is not a in the previous sentence, but the name is the same
a. push(4);
println!("{:?}", a); // print: [1, 2, 3, 4]
```
After "variable binding", the target memory is still the same block, but this memory can be modified through the newly bound a.

```rust
let mut a: &str = "abc"; //variable binding, a <=> memory area A("abc")
a = "xyz"; // bind to another memory area, a <=> memory area B("xyz")
println!("{:?}", a); //print: "xyz"
```
Don’t confuse the above situation, `a = "xyz"` means that the target resource bound by `a` has changed.

In fact, there are also const constants in Rust. There is no such thing as "binding" for constants, and they have the same meaning as constants in other languages:
```rust
const PI:f32 = 3.14;
```

The purpose of mutability is to strictly distinguish the mutability of bindings so that the compiler can optimize better and improve memory safety.

### **Advanced Copy Features**
In the previous section, we have a brief understanding of the Copy feature. Next, let's take a deeper look at this feature.
The Copy trait is defined in the standard library [std::marker::Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html ""):
```rust
pub trait Copy: Clone { }
```
Once a type implements the Copy trait, it means that the type can be copied by simply copying bits. From the previous knowledge, we know that "binding" has move semantics (ownership transfer), but once this type implements the Copy feature, it will first copy the content to a new memory area, and then bind the new memory area to this identifier.

** Under what circumstances can our custom type (such as a Struct, etc.) implement the Copy feature? **
As long as all attribute types of this type implement the Copy attribute, then this type can implement the Copy attribute.
For example:

```rust
struct Foo { // can implement the Copy feature
     a: i32,
     b: bool,
}

struct Bar { //The Copy property cannot be realized
     l: Vec<i32>,
}
```

Because the types `i32` and `bool` of `a` and `b` of `Foo` both implement the `Copy` feature, so `Foo` can also implement the Copy feature. But for `Bar`, its attribute `l` is of `Vec<T>` type, which does not implement the `Copy` feature, so `Bar` cannot implement the `Copy` feature.

**So how do we implement the `Copy` feature? **
There are two ways to achieve this.

1. **Use `derive` to let the Rust compiler automatically implement**

     ```rust
     #[derive(Copy, Clone)]
     struct Foo {
         a: i32,
         b: bool,
     }
     ```

     The compiler will automatically check whether all properties of `Foo` implement the `Copy` feature, and once the check is passed, it will automatically implement the `Copy` feature for `Foo`.

2. **Manually implement the `Clone` and `Copy` traits**

     ```rust
     #[derive(Debug)]
     struct Foo {
         a: i32,
         b: bool,
     }
     impl Copy for Foo {}
     impl Clone for Foo {
         fn clone(&self) -> Foo {
             Foo{a: self. a, b: self. b}
         }
     }
     fn main() {
         let x = Foo{ a: 100, b: true };
         let mut y = x;
         y.b = false;

         println!("{:?}", x); // print: Foo { a: 100, b: true }
         println!("{:?}", y); // print: Foo { a: 100, b: false }
     }

     ```

     From the results, we found that after `let mut y = x`, `x` did not have an inaccessible error due to ownership `move`.
     Since `Foo` inherits the `Copy` and `Clone` attributes, we implement these two attributes in the example.


### **Advanced move**
We learned from the previous sections that ownership transfers occur for `let` bindings, but `ownership` transfers behave differently depending on whether the resource type implements the `Copy` trait:
```rust
let x: T = something;
let y = x;
```
* The type `T` does not implement the `Copy` trait: ownership of `x` is transferred to `y`.
* The type `T` implements the `Copy` feature: copy the `resource` bound by `x` to a `new resource`, and bind the ownership of the `new resource` to `y`, `x` still owns the original Ownership of resources.

##### **move keyword**
The move keyword is often used in closures to force the closure to take ownership.

**Example 1:**
```rust
fn main() {
let x: i32 = 100;
let some_closure = move |i: i32| i + x;
let y = some_closure(2);
println!("x={}, y={}", x, y);
}
```
>Result: x=100, y=102

Note: Example 1 is quite special, so that the use of move will have no effect on the result, because the resource bound by `x` is of `i32` type, which belongs to `primitive type` and implements `Copy trait`, so in the closure When using `move`, `x` is copied first, and `x` of this clone is moved during move, so no error is reported when `println!` references `x` later.

**Example 2:**
```rust
fn main() {
let mut x: String = String::from("abc");
let mut some_closure = move |c: char| x.push(c);
let y = some_closure('d');
println!("x={:?}", x);
}
```
> **error:**
error: use of moved value: `x` [E0382]
<anon>:5 println!("x={:?}", x);

This is because the move keyword will move the ownership of the external variable in the closure to the package body, and the ownership transfer problem occurs, so `println` accessing `x` will be as wrong as above. If we remove `println`, it will compile and pass.

So, what if we want to still access x outside the package, that is, x does not lose ownership?
```rust
fn main() {
let mut x: String = String::from("abc");
{
     let mut some_closure = |c: char| x.push(c);
some_closure('d');
}
println!("x={:?}", x); // print successfully: x="abcd"
}
```
We just removed move. After removing move, `x` will be **mutable borrowed** in the package instead of "depriving" the ownership of `x`. Careful students also noticed that we added The scope of `{}` curly braces is to make **variable borrow** invalid after the scope ends, so that `println` can successfully access and print the content we expect.

We will explain in detail the knowledge of "**Borrowing borrowing**" in the next section.
