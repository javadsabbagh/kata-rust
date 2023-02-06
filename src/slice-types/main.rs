

fn main() {
    let mut s = String::from("Mr Barzin");

    let word = first_word(&s);

    println!("The first word is: {}", word);

    s.clear()
}

fn first_word(st: &String) -> &str {
    let bytes = st.as_bytes();

    for (i, &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &st[0..i];
        }
    }

    return &st[..]
}