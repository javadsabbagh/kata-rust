use uuid::Uuid;

fn main() {
    for _ in 1..=10 {
        let id = Uuid::new_v4().hyphenated().to_string();
        println!("{}", id);
    }

    println!("{:?}", Uuid::parse_str("d27cdb6e-ae6d-11cf-96b8-44455354000")); // Error(GroupLength)
    print!("{:?}", Uuid::parse_str("22d9673c-de24-11ed-915b-88a4c2e3226d"));  // Ok(Uuid));
}
