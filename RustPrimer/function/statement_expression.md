# Statements and expressions
   rust is an expression-based language, but it also has statements. Rust has only two kinds of statements: declaration statements and expression statements, and the others are expressions. Based on the fact that expressions are an important feature of functional languages, expressions always return a value.

## Declaration statement
   Rust declaration statements can be divided into two types, one is the variable declaration statement, and the other is the Item declaration statement.
   1. Variable declaration statement. Mainly refers to the `let` statement, such as:

  ```rust
  let a = 8;
  let b: Vec<f64> = Vec::new();
  let (a, c) = ("hi", false);
  ```
  
   Since let is a statement, you cannot assign a let statement to other values. The following form is wrong:
  
  ```rust
  let b = (let a = 8);
  ```
  
   The rustc compiler will give an error message: ![error](../images/function-statement-expression.png)

   2. Item statement. A declaration of a function, structure, type, static, trait, implementation, or module. These declarations can be nested in arbitrary blocks. Regarding the Item declaration, the description in the Rust Reference is as follows:
   > An item declaration statement has a syntactic form identical to an item declaration within a module. Declaring an item — a function, enumeration, structure, type, static, trait, implementation or module — locally within a statement block is simply a way of restricting its scope to a narrow region containing all of its uses; it is otherwise identical in meaning to declaring the item outside the statement block.

   Of course, we cannot expand on how these items are declared here. For details, please refer to other relevant chapters of RustPrimer.

## expression statement
   An expression statement consists of an expression and a semicolon, that is, adding a semicolon after the expression turns an expression into a statement. Therefore, there are as many expression statements as there are expressions.

   __rust has many kinds of expressions:__
   * literal expression

  ```rust
  ();        // unit type
  "hello";   // string type
  '1';       // character type
  15;         // integer type
  ```

   * Tuple expression:

  ```rust
  (0.0, 4.5);
  ("a", 4usize, true);
  ```
  
   One-element tuples are generally not used, but if you insist, rust allows it, but you need to add a comma after the element:
  
  ```rust
  (0,); // single-element tuple
  (0); // zero in parentheses
  ```

   * Structure expression (structure expression)
   As structs come in many forms, struct expressions also come in many forms.
  
  ```rust
  Point {x: 10.0, y: 20.0};
  TuplePoint(10.0, 20.0);
  let u = game::User {name: "Joe", age: 35, score: 100_000};
  some_fn::<Cookie>(Cookie);
  ```
  
   Structure expressions are generally used to construct a structure object, which can be constructed on the basis of another object in addition to the above form of building from zero:
  
  ```rust
  let base = Point3d {x: 1, y: 2, z: 3};
  Point3d {y: 0, z: 10, .. base};
  ```

   * Block expression (block expression):
   A block expression is a set of expressions enclosed in curly braces `{}`, and the expressions are usually separated by semicolons. The value of the block expression is the value of the last expression.
  
   ```rust
   let x: i32 = { println!("Hello."); 5 };
   ```
  
   If it ends with a statement, the block expression evaluates to `()`:
  
   ```rust
   let x: () = { println!("Hello."); };
   ```

   * range expression (range expression):
   Range objects (variant of `std::ops::Range`) can be constructed using the range operator `..`:
  
  ```rust
  1..2;   // std::ops::Range
  3..;    // std::ops::RangeFrom
  ..4;    // std::ops::RangeTo
  ..;     // std::ops::RangeFull
  ```

   * if expression (if expression):

  ```rust
  let a = 9;
  let b = if a%2 == 0 {"even"} else {"odd"};
  ```

   * In addition to the above, there are many more, such as:
     + path expression
     + mehond-call expression
     + field expression
     + array expression
     + index expression
     + unary operator expression
     + binary operator expression
     + return expression
     + grouped expression
     + match expression
     + if expression
     + lambda expression
     + ... ...

   It cannot be expanded in detail here, readers can go to [Rust Reference][1] to check.
   [1]: http://doc.rust-lang.org/reference.html#statements-and-expressions

> #### Some examples in the above expression statements are quoted from [Rust Reference][ref]
   [ref]: http://doc.rust-lang.org/reference.html
   
