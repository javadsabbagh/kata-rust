#Cow

Literally translated as cows! joke.
`Cow` is an enumerated type introduced by `use std::borrow::Cow;`. Its definition is `Clone-on-write`, that is, clone on write. Essentially a smart pointer.

It has two optional values:
- `Borrowed`, used to wrap object references (universal references);
- `Owned`, used to wrap the owner of the object;

`Cow` provides

1. Immutable access to this object (for example, you can directly call the original immutable method of this object);
2. If you need to modify this object, or need to obtain the ownership of this object, `Cow` provides a method for cloning, and avoids repeated cloning.

`Cow` is designed to improve performance (reduce replication) and increase flexibility, because in most cases, business scenarios are more read and less write. Using `Cow`, it can be implemented in a unified and standardized form, and the object is copied only once when it needs to be written. This may greatly reduce the number of copies.

It has the following points to master:

1. `Cow<T>` can directly call the immutable method of `T`, because `Cow` is an enumeration that implements `Deref`;
2. When you need to write `T`, you can use the `.to_mut()` method to get a mutable borrow of the value with ownership;
     1. Note that calling `.to_mut()` does not necessarily produce a clone;
     2. In the case of already having ownership, calling `.to_mut()` is valid, but will not generate a new clone;
     3. Multiple calls to `.to_mut()` will only produce one clone.
3. When you need to write `T`, you can use `.into_owned()` to create a new object with ownership. This process often means copying memory and creating a new object;
     1. If the previous value in `Cow` is a borrowed state, calling this operation will perform a clone;
     2. This method, the parameter is `self` type, it will "eat" the original object, the life cycle of the original object will end after calling, and it cannot be called multiple times on `Cow`;


## Example

`.to_mut()` example

```rust
use std::borrow::Cow;

let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);

let hello = cow. to_mut();

assert_eq!(hello, &[1, 2, 3]);
```

`.into_owned()` Example

```rust
use std::borrow::Cow;

let cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);

let hello = cow. into_owned();

assert_eq!(vec![1, 2, 3], hello);
```

Comprehensive example

```rust
use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
     for i in 0..input.len() {
         let v = input[i];
         if v < 0 {
             // clones into a vector the first time (if not already owned)
             input.to_mut()[i] = -v;
         }
     }
}
```

## Application example of `Cow` on function return value

Topic: Write a function that filters out all space characters in the input string and returns the filtered string.

For this simple problem, without thinking, we can quickly write code:

```rust
fn remove_spaces(input: &str) -> String {
    let mut buf = String::with_capacity(input. len());

    for c in input. chars() {
       if c != ' ' {
          buf. push(c);
       }
    }

    buf
}
```

When designing function input parameters, we will pause for a moment. Here, should we use `&str` or `String`? After thinking about it, considering the performance, we have the following conclusions:

1. If `String` is used, when this function is called externally,
     1. If the external string is `&str`, it needs to be cloned before calling this function;
     2. If the external string is `String`, then it can call this function without cloning. However, once called, the ownership of the external string will be `move` into this function, and subsequent external codes will no longer be able to use the original string.
2. If `&str` is used, the above two problems do not exist. But you may encounter life cycle problems, you need to pay attention.

Continuing to analyze the above example, we find that in the function body, a new string object is generated and copied.

Let's take a closer look at the business requirements. In the worst case, if there are no whitespace characters in the string, it is better to return it directly. In this case, it is completely wasteful to make such a copy of the object.

So we wanted to improve this algorithm. Soon, I encountered another problem. The return value is `String`. No matter what I want to convert `&str` into `String` and return, I always have to go through a copy. So we were about to give up.

Well, Mr. `Cow` is out now. Dai Niu quickly wrote the following code:

```rust
use std::borrow::Cow;

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
     if input. contains(' ') {
         let mut buf = String::with_capacity(input. len());

         for c in input. chars() {
             if c != ' ' {
                 buf. push(c);
             }
         }

         return Cow::Owned(buf);
     }

     return Cow::Borrowed(input);
}

```

It perfectly solves the problem of conflict between business logic and return value type. This example can be savored carefully.

External programs, after getting the return value of `Cow`, just use it according to the characteristics of `Cow` we described above.
