# Lifecycle ( Lifetime )


The following is an example of resource borrowing:

```rust
fn main() {
    let a = 100_i32;

    {
        let x = &a;
    } // x end of scope
    println!("{}", x);
}
```

When compiling, we will see a serious error message:

> error: unresolved name `x`.

The error means "unable to parse `x` identifier", that is, `x` cannot be found, this is because like many programming languages, there is also a concept of scope in Rust, when the resource leaves the scope, the resource's The memory will be released and reclaimed, and it will be destroyed when the borrow/reference leaves the scope, so `x` cannot be accessed outside the scope after leaving its own scope.


The above involves several concepts:

* **Owner**: The owner of the resource `a`
* **Borrower**: The resource borrower `x`
* **Scope**: Scope, the validity period of the resource being borrowed/referenced


To emphasize, whether it is the owner of the resource or the borrowing/reference of the resource, there is a valid survival time or interval. This time interval is called **life cycle**, and it can also be directly used as **Scope scope** to understand.

So the life cycle/scope diagram in the above example code is as follows:


```
             { a { x } * }
Owner a: |________________________|
borrower x: |____| x = &a
   access x: | failed: access x
```

As you can see, the lifetime of the borrower `x` is a **subset** of the lifetime of the resource owner `a`. However, the life cycle of `x` ends and is destroyed at the first `}`, and a serious error will occur if it is accessed again in the next `println!`.

Let's fix the example above:

```rust
fn main() {
let a = 100_i32;

{
let x = &a;
println!("{}", x);
} // x end of scope

}
```

Here we just put `println!` in the middle `{}`, so that `x` can be accessed normally during the life cycle of `x`, and the Lifetime diagram at this time is as follows:

```
            {    a    {    x    *    }    }
owner a:         |________________________|
borrower x:                   |_________|       x = &a
  visit x:                        |            OK：visit x
```



## Implicit Lifetime
We often encounter functions whose parameters or return values are reference types:

```rust
fn foo(x: &str) -> &str {
	x
}
```

The above function is not very useful in practical applications. The `foo` function only accepts a parameter of `&str` type (`x` is a borrow of a `string` type resource `Something`), and returns a reference to the resource` A new borrow of Something`.

In fact, the above function contains the implicit life cycle name, which is automatically deduced by the compiler, which is equivalent to:

```rust
fn foo<'a>(x: &'a str) -> &'a str {
	x
}
```

Here, the Lifetime of the constraint return value must be greater than or equal to the Lifetime of the parameter `x`. The following function writing is also legal:

```rust
fn foo<'a>(x: &'a str) -> &'a str {
	"hello, world!"
}
```

why? This is because the type of the string "hello, world!" is `&'static str`, we know that the Lifetime of the `static` type is the running cycle of the entire program, so she is more than the Lifetime`'a` of any incoming parameter Both must be long, that is, `'static >= 'a` is satisfied.


In the above example, Rust can automatically deduce Lifetime, so the programmer does not need to explicitly specify Lifetime `'a`.

What is `'a`? It is the identifier of Lifetime, where `a` can also use `b`, `c`, `d`, `e`, ..., or even `this_is_a_long_name`, etc. Of course, it is not recommended in actual programming Using such lengthy identifiers can seriously reduce the readability of the program. The `<'a>` behind `foo` is the declaration of Lifetime, and multiple declarations can be made, such as `<'a, 'b>` and so on.

In addition, unless the compiler cannot automatically deduce Lifetime, it is not recommended to explicitly specify the Lifetime identifier, which will reduce the readability of the program.

## Explicit Lifetime
What happens when the input parameter is multiple borrows/references?

```rust
fn foo(x: &str, y: &str) -> &str {
if true {
x
} else {
the y
}
}
```

Compile again at this time, not so lucky:

```
error: missing lifetime specifier [E0106]
fn foo(x: &str, y: &str) -> &str {
                            ^~~~
```

The compiler tells us that we need to explicitly specify the Lifetime identifier, because at this time, the compiler cannot deduce whether the Lifetime of the returned value should be longer than `x` or longer than `y`. Although we used `if true` in the function to confirm that `x` must be returned, but you must know that the compiler checks at compile time, not runtime, so all input parameters and return values will be checked at the same time during compilation .

The fixed code is as follows:

```rust
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
	if true {
		x
	} else {
		y
	}
}
```

## Lifetime derivation

To deduce whether Lifetime is legal, two points must be clarified first:

* which input values the output value (also known as the return value) depends on
* The Lifetime of the input value is greater than or equal to the Lifetime of the output value (accurately: a subset, not greater than or equal to)

**Lifetime derivation formula:**
When the output value R depends on the input value X Y Z ..., if and only if the Lifetime of the output value is a subset of the Lifetime intersection of all input values, the lifetime is legal.

```
Lifetime(R) ⊆ ( Lifetime(X) ∩ Lifetime(Y) ∩ Lifetime(Z) ∩ Lifetime(...) )
```

For example 1:

```rust
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
	if true {
		x
	} else {
		y
	}
}
```

Because the return value depends on both the input parameters `x` and `y`, so

```
Lifetime(return value) ⊆ ( Lifetime(x) ∩ Lifetime(y) )

	Right now:

'a ⊆ ('a ∩ 'a) // holds
```


#### Define multiple Lifetime identifiers
Then let's continue to look at a more complex example, defining multiple Lifetime identifiers:

```rust
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
	if true {
		x
	} else {
		y
	}
}
```

Look at the compilation first, and report an error again:

```
<anon>:5:3: 5:4 error: cannot infer an appropriate lifetime for automatic coercion due to conflicting requirements [E0495]
<anon>:5 		y
         		^
<anon>:1:1: 7:2 help: consider using an explicit lifetime parameter as shown: fn foo<'a>(x: &'a str, y: &'a str) -> &'a str
<anon>:1 fn bar<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
<anon>:2 	if true {
<anon>:3 		x
<anon>:4 	} else {
<anon>:5 		y
<anon>:6 	}
```

The compiler says that it cannot correctly derive the Lifetime of the return value, and the reader may wonder, "Didn't we already specify the Lifetime of the return value as `'a`?".

Here we can also deduce it through the life cycle derivation formula:

Because the return value depends on both `x` and `y`, so

```
Lifetime(return value) ⊆ ( Lifetime(x) ∩ Lifetime(y) )

	Right now:

'a ⊆ ('a ∩ 'b) // false
```

Obviously, we can't guarantee the above at all.

So, in this case, we can explicitly tell the compiler that `'b` is longer than `'a` (`'a` is a subset of `'b`), only when defining Lifetime, in ` Add `: 'a` after 'b`, which means that `'b` is longer than `'a`, and `'a` is a subset of `'b`:

```
fn foo<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
if true {
x
} else {
the y
}
}
```

Here we continue to derive according to the formula:

```
Condition: Lifetime(x) ⊆ Lifetime(y)
Derivation: Lifetime (return value) ⊆ ( Lifetime(x) ∩ Lifetime(y) )

	Right now:

Condition: 'a ⊆ 'b
Derivation: 'a ⊆ ('a ∩ 'b) // established
```

The above is established, so it can be compiled and passed.

#### Derivation summary
Through the above study, I believe that you can easily complete the derivation of Lifetime. In short, remember two points:

1. On which input values the output value depends.
2. Deriving the formula.



## Lifetime in struct
We discussed more about the application of Lifetime in functions above, and Lifetime is equally important in `struct`.

Let's define a `Person` structure:

```rust
struct Person {
age: &u8,
}
```

When compiling we get an error:

```
<anon>:2:8: 2:12 error: missing lifetime specifier [E0106]
<anon>:2 age: &str,
```

The reason why the error is reported is because Rust needs to ensure that the Lifetime of `Person` will not be longer than its `age` borrowed, otherwise there will be a serious memory problem of `Dangling Pointer`. So we need to declare Lifetime for `age` borrowing:

```rust
struct Person<'a> {
age: &'a u8,
}
```

There is no need to be confused about the `<'a>` behind `Person`. The `'a` here does not refer to the Lifetime of the `struct` of `Person`, it is just a generic parameter. How many `struct`s can there be? A Lifetime parameter is used to constrain different `field`s, and the actual Lifetime should be a subset of the intersection of all `field`Lifetimes. For example:

```
fn main() {
let x = 20_u8;
let stormgbs = Person {
age: &x,
};
}
```

Here, the schematic diagram of the life cycle/Scope is as follows:

```
                  {   x    stormgbs      *     }
owner x:              |________________________|
owner stormgbs:                |_______________|  'a
borrower stormgbs.age:         |_______________|  stormgbs.age = &x
```

Since `<'a>` is used as the generic parameter of `Person`, you also need to add `<'a>` when implementing the method for `Person`, otherwise:

```rust
impl Person {
	fn print_age(&self) {
		println!("Person.age = {}", self.age);
	}
}
```

Error:

```
<anon>:5:6: 5:12 error: wrong number of lifetime parameters: expected 1, found 0 [E0107]
<anon>:5 impl Person {
              ^~~~~~
```

**The correct approach is**:

```rust
impl<'a> Person<'a> {
	fn print_age(&self) {
		println!("Person.age = {}", self.age);
	}
}
```

This can be done after adding `<'a>`. Readers may wonder why there is no need to add `'a` in `print_age`? That's a good question. Because the output parameter of `print_age` is `()`, that is, it does not depend on any input parameters, so the compiler does not need to care about and derive Lifetime at this time. Even `fn print_age(&self, other_age: &i32) {...}` can compile.

** What if there is an output value (borrowed) in the method of `Person`? **

```rust
impl<'a> Person<'a> {
	fn get_age(&self) -> &u8 {
		self.age
	}
}
```

The output value of the `get_age` method depends on an input value `&self`. In this case, the Rust compiler can automatically deduce it as:

```rust
impl<'a> Person<'a> {
	fn get_age(&'a self) -> &'a u8 {
		self.age
	}
}
```

**What if the output value (borrowed) depends on multiple input values? **


```rust
impl<'a, 'b> Person<'a> {
	fn get_max_age(&'a self, p: &'a Person) -> &'a u8 {
		if self.age > p.age {
			self.age
		} else {
			p.age
		}
	}
}
```

Similar to the previous Lifetime derivation chapter, when the return value (borrowed) depends on multiple input values, you need to explicitly declare Lifetime. It is the same as the function Lifetime.

**other**

Lifetime theoretical knowledge is the same whether in a function or in a `struct`, or even in an `enum`. I hope that everyone can slowly experience and absorb it, and learn from it.

## Summarize

Rust manages memory almost perfectly in an efficient and safe way through ownership, borrowing, and lifetime. There is no load and safety of manually managing memory, and no program halting issues caused by GC.

