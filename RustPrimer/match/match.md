# match keyword
Pattern matching, which often appears in functional programming languages, provides a simple and easy deconstruction capability for its complex type system. For example, extracting data from enum and other data structures, etc., but in writing, it is relatively complicated. Let's look at an example:

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
```

This is a program with no practical significance, but it can clearly express the usage of match. Seeing this, you can definitely think of a common control statement - `switch`. That's right, match can play the same role as switch. However, there are a few points to note:

1. For the matches listed in match, all possibilities must be exhaustively listed. Of course, you can also use the symbol **_** to represent all other possible situations, which is similar to the `default` statement in switch.
2. Each branch of match must be an expression, and, unless a branch must trigger a panic, the final return value type of all expressions of these branches must be the same.

Regarding the second point, some students may not understand. Let's put it this way, you can regard match as an expression as a whole. Since it is an expression, you can definitely get its result. Therefore, this result can of course be assigned to a variable.
Look at the code:

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    // let d_panic = Direction::South;
    let d_west = Direction::West;
    let d_str = match d_west {
        Direction::East => "East",
        Direction::North | Direction::South => {
            panic!("South or North");
        },
        _ => "West",
    };

    println!("{}", d_str);
}
```

## A first look at deconstruction

Another very important role of match is to deconstruct the existing data structure and easily take out the data part.
For example, the following are more common examples:

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let action = Action::Say("Hello Rust".to_string());
    match action {
        Action::Say(s) => {
            println!("{}", s);
        },
        Action::MoveTo(x, y) => {
            println!("point from (0, 0) move to ({}, {})", x, y);
        },
        Action::ChangeColorRGB(r, g, _) => {
            println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                r, g,
            );
        }
    }
}
```

Someone said, from this point of view, I don't think match is so amazing! Don't worry, please read the next section ——>[pattern](pattern.md)
