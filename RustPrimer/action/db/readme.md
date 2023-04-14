#rust database operation

When programming, we rely on the database to store the corresponding data. Many programming languages support the operation of the database, so of course we can use Rust to operate the database.

However, when I operated it myself, I found many problems, mainly because I did not understand the things that Rust should pay attention to when operating the database, which wasted a lot of time when querying data.
I will give some demonstrations about the specific pits I encountered, so that everyone can avoid these situations.

First use Rust to manipulate PostgreSQL, because PostgreSQL is my favorite database.

First create a new project `cargo new db --bin`

Add `postgres` in cargo.toml as follows:


``` rust
[package]
name = "db"
version = "0.1.0"
authors = ["vagrant"]

[dependencies]
postgres="*"
```


Of course, we still perform the simplest operation, directly paste and copy, [code source](https://github.com/sfackler/rust-postgres#overview)

``` rust

extern crate postgres;

use postgres::{Connection, SslMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None)
            .unwrap();

    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();

    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person {}", person.name);
    }
}

```

These simple, of course, are not what we want, what we want is to be able to do some layering, that is
Some basic functions are logically divided, instead of doing everything in a main function.

## Create lib.rs file

View the file from top to bottom:

1. First import various libraries of postgres
2. Create a Person struct, according to the required fields and types.
3. Create a connection function that returns a connection object.
4. Create an insert function to insert data
5. Create a query function to query data
6. Create a query function to query all data.

Of course, these functions have certain functional limitations.

``` rust

extern crate postgres;

use postgres::{Connection, SslMode};
use postgres::types::FromSql;
use postgres::Result as PgResult;


struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}


pub fn connect() -> Connection{
    let dsn = "postgresql://postgres:2015@localhost/rust_example";
    Connection::connect(dsn, SslMode::None).unwrap()
}

pub fn insert_info(conn : &Connection,title : &str, body: &str){

    let stmt = match conn.prepare("insert into blog (title, body) values ($1, $2)") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {:?}", e);
            return;
        }
    };
        stmt.execute(&[&title, &body]).expect("Inserting blogposts failed");
}


pub fn query<T>(conn: &Connection,query: &str) ->PgResult<T>
        where T: FromSql {
            println!("Executing query: {}", query);
            let stmt = try!(conn.prepare(query));
            let rows = try!(stmt.query(&[]));
            &rows.iter().next().unwrap();
            let row = &rows.iter().next().unwrap();
                //rows.iter().next().unwrap()
            row.get_opt(2).unwrap()

}

pub fn query_all(conn: &Connection,query: &str){
            println!("Executing query: {}", query);
            for row in &conn.query(query,&[]).unwrap(){
                let person = Person{
                    id: row.get(0),
                    name: row.get(1),
                    data: row.get(2)
            };
            println!("Found person {}", person.name);
            }

}

```

Then call the corresponding function code in main.rs as follows
1. extern db, import db, that is, import the project itself
2. use db Use the functions that can be introduced in db
3. Define the blog, since the personal blog table is created by yourself, so if an error is reported that the table does not exist, you need to create it yourself
4. Use the functions defined in lib to perform some basic operations

``` rust
extern crate postgres;
extern crate db;

use postgres::{Connection, SslMode};

use db::*;

struct Blog {
    title: String,
    body:  String,
}

fn main() {
    let conn:Connection=connect();

    let blog = Blog{
        title: String::from("title"),
        body: String::from("body"),
    };
    let title = blog.title.to_string();
    let body = blog.body.to_string();
    insert_info(&conn,&title,&body);

   for row in query::<String>(&conn,"select * from blog"){
        println!("{:?}",row);
    }
    let sql = "select * from person";
    query_all(&conn,&sql);
}

```

the pit I encountered

- When creating a connection function, the connection must have a return value, so the type of the return value must be specified,
For a person who writes Python, I think it is painful, I want to match according to the official way of writing
After a while, it is found that multiple return values may be generated. It directly fails to compile at compile time, so in the end
I used unwrap to solve the problem, but I still haven't learned how to return a function with multiple values
Define the return value

- I followed the documentation when using `&conn.query(query,&[]).unwrap()`, the documentation says
What is returned is an iterable data, that is to say, I can use the for loop to print the data,
But found that it can't be achieved:

``` rust

pub fn query_all(conn: &Connection,query: &str){
            println!("Executing query: {}", query);
            for row in &conn.query(query,&[]).unwrap(){
                  println!("Found person {:?}", row.get_opt(1));
            }
}

```

报错如下：

``` rust
vagrant@ubuntu-14:~/tmp/test/rustprimer/db$ cargo run
   Compiling db v0.1.0 (file:///home/vagrant/tmp/test/rustprimer/db)
src/lib.rs:53:37: 53:47 error: unable to infer enough type information about `_`; type annotations or generic parameter binding required [E0282]
src/lib.rs:53   println!("Found person {:?}", row.get_opt(1));
                                                  ^~~~~~~~~~
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/lib.rs:53:3: 53:49 note: in this expansion of println! (defined in <std macros>)
src/lib.rs:53:37: 53:47 help: run `rustc --explain E0282` to see a detailed explanation
error: aborting due to previous error
Could not compile `db`.

```

Then I checked all the functions of the postgres module, tried countless methods, and still couldn't solve it.

Maybe my eyes are high and my hands are low. If I read the relevant tutorials of Rust from the beginning, I may find this problem very early.
It may also be because I am used to writing Python, causing myself to use inherent thinking to look at problems and dig into the corners, so
This leads to such problems and wastes a lot of time.

- Change your thinking and treat yourself as a brand new novice. It is necessary to use existing thinking to learn a new language, but also not
The language that I am very proficient in solidifies my thinking.
