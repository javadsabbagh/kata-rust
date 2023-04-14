# AsRef and AsMut

Below `std::convert`, there are two other Traits, `AsRef/AsMut`, which function to cooperate with generics to perform automatic type conversion when performing reference operations. This can make the code of some scenes clear and beautiful, and it is convenient for everyone to develop.

## AsRef<T>

`AsRef` provides a method `.as_ref()`.

For an object `foo` of type `T`, if `T` implements `AsRef<U>`, then `foo` can execute the `.as_ref()` operation, namely `foo.as_ref()`. As a result of the operation, we get a new reference of type `&U`.

Note:

1. Unlike `Into<T>`, `AsRef<T>` is just a type conversion, and the `foo` object itself is not consumed;
2. `T` in `T: AsRef<U>` can accept resource owner (owned) type, shared reference (shared reference) type, mutable reference (mutable reference) type.

Here is a simple example:

```rust
fn is_hello<T: AsRef<str>>(s: T) {
    assert_eq!("hello", s.as_ref());
}

let s = "hello";
is_hello(s);

let s = "hello".to_string();
is_hello(s);
```

Because both `String` and `&str` implement `AsRef<str>`.


## AsMut<T>

`AsMut<T>` provides a method `.as_mut()`. It is the mutable reference version of `AsRef<T>`.

For an object `foo` of type `T`, if `T` implements `AsMut<U>`, then `foo` can execute the `.as_mut()` operation, namely `foo.as_mut()`. As a result of the operation, we get a mutable reference of type `&mut U`.

Note: During conversion, `foo` will be borrowed by mutable.
