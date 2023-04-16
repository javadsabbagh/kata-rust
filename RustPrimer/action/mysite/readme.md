# rust web development

Since rust is a system-level programming language, it can of course also be used to develop the web, but for an ordinary person like me, I certainly cannot write a web from scratch.
The server must rely on the existing rust web development framework to complete web development.

The currently well-known rust frameworks are iron and nickel, and we will write simple tutorials for both of them.

##iron

Continuing from the previous article, use cargo to obtain third-party libraries. `cargo new mysite --bin`

Add iron dependency in cargo.toml,

```toml
[dependencies]
iron = "*"
```

Then build will download the dependencies to the local `cargo build`

If an ssl error is reported, you may need to install the ssl development library for linux.

Let's start with hello world first, and continue to copy the official example:

``` rust
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}
```

then run

`cargo run`

Use curl to directly access your website.

`curl localhost:3000`

`Hello World!`

After a closer look, I found that this example is very nonsensical. For me who is used to writing python, I am really not used to it.
Simply look at:

`iron::new().http("localhost:3000").unwrap()`
This sentence is the basic definition of the server, new inside is a [rust lambda expression](https://doc.rust-lang.org/book/closures.html)

```rust
let plus_one = |x: i32| x + 1;

assert_eq!(2, plus_one(1));
```

How to use it specifically, you can ignore it for the time being, because you only need to know how to complete the web, because I don't know how to do it either. .
Combined with the json processing in the previous chapter, let’s take a look at how the web interface returns json, of course rustc_serialize must be put in cargo.toml

*The following code directly refers to the open source code [address](https://github.com/brson/httptest#lets-make-a-web-service-and-client-in-rust)*

```rust
extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Greeting {
    msg: String
}

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
```

Execute cargo run and use curl to test the results:

```
curl localhost:3000
{"msg":"Hello, World"}
```

Of course, more business needs can be achieved by controlling your own json.

Now that we have json, if we want multiple routes or something, it’s over, so it’s impossible, we need to think about how to realize the customization of routes

Go directly to the code without talking, and also add a dependency on router in your cargo.toml file.

``` rust
extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world);
    router.post("/set", set_greeting);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let payload = request.body.read_to_string();
        let request: Greeting = json::decode(payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
```

This time, the implementation of routing and the acquisition of data sent by the client are added, with get and post, so now a basic api website has been completed. but
Not all websites are accessed through APIs, which also require HTML template engines and direct return to static pages. etc.

```
vagrant@ubuntu-14:~/tmp/test/rustprimer/mysite$ cargo build
   Compiling mysite v0.1.0 (file:///home/vagrant/tmp/test/rustprimer/mysite)
src/main.rs:29:36: 29:52 error: no method named `read_to_string` found for type `iron::request::Body<'_, '_>` in the current scope
src/main.rs:29         let payload = request.body.read_to_string();
                                                  ^~~~~~~~~~~~~~~~
src/main.rs:29:36: 29:52 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
src/main.rs:29:36: 29:52 help: candidate #1: use `std::io::Read`
error: aborting due to previous error
Could not compile `mysite`.
```

There was a compilation error, too bad, it was prompted that there is no read_to_string method, then I went to the documentation to check and found [read_to_string method](http://ironframework.io/doc/iron/request/struct.Body.html)
Look at the prompt information again

```
src/main.rs:29:36: 29:52 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
src/main.rs:29:36: 29:52 help: candidate #1: use `std::io::Read`
```

Let's add a `std::io::Read`, if you have manipulated the file, you must know how to write it, add it, it should be able to pass, or continue to make mistakes, look at the error

```
   Compiling mysite v0.1.0 (file:///home/vagrant/tmp/test/rustprimer/mysite)
src/main.rs:30:36: 30:52 error: this function takes 1 parameter but 0 parameters were supplied [E0061]
src/main.rs:30         let payload = request.body.read_to_string();
                                                  ^~~~~~~~~~~~~~~~
src/main.rs:30:36: 30:52 help: run `rustc --explain E0061` to see a detailed explanation
src/main.rs:31:46: 31:53 error: mismatched types:
 expected `&str`,
    found `core::result::Result<usize, std::io::error::Error>`
(expected &-ptr,
    found enum `core::result::Result`) [E0308]
src/main.rs:31         let request: Greeting = json::decode(payload).unwrap();
                                                            ^~~~~~~
src/main.rs:31:46: 31:53 help: run `rustc --explain E0308` to see a detailed explanation
src/main.rs:30:36: 30:52 error: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements [E0495]
src/main.rs:30         let payload = request.body.read_to_string();
                                                  ^~~~~~~~~~~~~~~~
src/main.rs:29:5: 35:6 help: consider using an explicit lifetime parameter as shown: fn set_greeting<'a>(request: &mut Request<'a, 'a>) -> IronResult<Response>
src/main.rs:29     fn set_greeting(request: &mut Request) -> IronResult<Response> {
src/main.rs:30         let payload = request.body.read_to_string();
src/main.rs:31         let request: Greeting = json::decode(payload).unwrap();
src/main.rs:32         let greeting = Greeting { msg: request.msg };
src/main.rs:33         let payload = json::encode(&greeting).unwrap();
src/main.rs:34         Ok(Response::with((status::Ok, payload)))
               ...
error: aborting due to 3 previous errors
Could not compile `mysite`.

```

The first sentence reminds us that this read_to_string() must have at least one parameter, but we have not provided any of them.
Let's take a look at the usage of [read_to_string](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string)

``` rust

se std::io;
use std::io::prelude::*;
use std::fs::File;

let mut f = try!(File::open("foo.txt"));
let mut buffer = String::new();

try!(f.read_to_string(&mut buffer));

```

The usage is relatively simple, let's modify the function just now:

```
fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload);
        let request: Greeting = json::decode(&payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }
```

Read the string from the request, store the read result in the payload, and then operate it, compile it and run it, and use curl to submit a post data

```
$curl -X POST -d '{"msg":"Just trust the Rust"}' http://localhost:3000/set
{"msg":"Just trust the Rust"}
```

iron is basically over
Of course, there is also how to use the html template engine, that is, just look at the document directly.

##[nickel](http://nickel.rs/)

Of course, since it is a web framework, it must be capable of iron and nickel, so let's see how to make a hello and return an html
page of

Similarly we create `cargo new site --bin`, then add nickel to cargo.toml, `cargo build`

``` rust

#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:6767");
}
```

Simply put, that's what happened.

1. Introduced the nickel macro
2. Initialize Nickel
3. Call utilize to define the routing module.
4. `router!` macro, the incoming parameter is the get method and the corresponding path, "\*\*" is the full path match.
5. listen start server

[Of course we need to introduce information about html templates] (http://nickel.rs/#easy-templating)

```rust
#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("name", "user");
        return response.render("site/assets/template.tpl", &data);
    });

    server.listen("127.0.0.1:6767");
}

```

You can compile the above information, use curl to see if it appears

```
$ curl http://127.0.0.1:6767
Internal Server Error
```

Looking at the documentation, I found no problems. I quickly changed the name of a folder, and I also created this folder.
Then I wondered if the server had written the directory to death? So correct the above path to this, and the problem is solved.

```rust
return response.render("examples/assets/template.tpl", &data);
```

Let's look at the directory structure

```
.
|-- Cargo.lock
|-- Cargo.toml
|-- examples
|   `-- assets
|       `-- template.tpl
|-- src
|   `-- main.rs

```
