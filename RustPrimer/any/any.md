# Any and reflection

Students who are familiar with Java must still remember the reflection ability of Java. Similarly, Rust also provides the ability of runtime reflection. However, there is a little difference here, because Rust does not have a VM or a Runtime, so the reflection it provides is more like a compile-time reflection.

Because, Rust can only reflect variables (constants) in the ``static` lifetime!

## for example

We will have such a requirement to load configuration files in certain paths. We may provide a configuration file path, well, this is a string (`String`). But what if I want to pass in paths to multiple configuration files? As it should be, we pass in an array.

This is bad... Rust doesn't support overloading! So someone simply wrote two functions~~!

Actually no need... we just need to write like this...

```rust
use std::any::Any;
use std::fmt::Debug;

fn load_config<T:Any+Debug>(value: &T) -> Vec<String>{
    let mut cfgs: Vec<String>= vec![];
    let value = value as &Any;
    match value.downcast_ref::<String>() {
        Some(cfp) => cfgs.push(cfp.clone()),
        None => (),
    };

    match value.downcast_ref::<Vec<String>>() {
        Some(v) => cfgs.extend_from_slice(&v),
        None =>(),
    }

    if cfgs.len() == 0 {
        panic!("No Config File");
    }
    cfgs
}

fn main() {
    let cfp = "/etc/wayslog.conf".to_string();
    assert_eq!(load_config(&cfp), vec!["/etc/wayslog.conf".to_string()]);
    let cfps = vec!["/etc/wayslog.conf".to_string(),
                    "/etc/wayslog_sec.conf".to_string()];
    assert_eq!(load_config(&cfps),
               vec!["/etc/wayslog.conf".to_string(),
                    "/etc/wayslog_sec.conf".to_string()]);
}
```

Let's focus on analyzing the middle function:

```rust
fn load_config<T:Any+Debug>(value: &T) -> Vec<String>{..}
```

First, this function accepts a generic `T` type, `T` must implement `Any` and `Debug`.

Some students may have questions here. Didn’t you say that you can only reflect variables in the ``static` life cycle? Let's look at the `Any` restriction:

```rust
pub trait Any: 'static + Reflect {
    fn get_type_id(&self) -> TypeId;
}
```

Look, `Any` specifies its life cycle when it is defined, and `Reflect` is a Marker, and all Rust types will implement it by default! Note that this is not all primitive types, but all types.

Okay, continue, since we can't judge the type of the parameter passed in, we can only reflect the type from runtime.

```rust
let value = value as &Any;
```

First, we need to convert the incoming type into a `trait Object`, of course, you can use `UFCS` if you like, refer to the appendix at the end of this chapter.

In this way, value can be called an Any. Then, we use `downcast_ref` to perform type inference. If the type inference succeeds, the value will be converted to the original type.

Some students are a little confused when they see this, why do you switch back after switching to Any?

In fact, the conversion to Any is to have the opportunity to obtain its type information, and the conversion back is to use the value itself.

Finally, we apply different processing logic to different types. Finally, a reflection function is complete.

## Talk about the attention

It should be noted that the reflection capabilities provided by Rust itself are not very powerful. Relatively speaking, it can only be used as an auxiliary means. Moreover, the limitation that it can only reflect on ``static` cycles does limit its performance. Another thing to note is that Rust's reflection can only be used for type inference, and it must not be used for interface assertion!

What, you ask why? Because I can't write it...
