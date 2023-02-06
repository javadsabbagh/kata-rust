

fn main() {
    let mut s = String::from("Mr Barzin");
    let word = first_word(&s);  // immutable borrowing
    println!("The first word is: {}", word); // immutable borrowing
    s.clear();  // mutable borrowing. Note: it must be after immutable borrowing, unless it'll get compile eeor
    println!("{}", s);


    // array slice sample
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..=3];
    assert_eq!(slice, &[2, 3, 4])
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