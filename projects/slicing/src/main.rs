fn main() {
    let string = String::from("That's all folks!");
    println!("{:?}", first_word(&string));
}

// Returning the first word
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
