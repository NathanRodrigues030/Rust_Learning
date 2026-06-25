fn main() {
    reference();
    mutable_reference();
}

// To not move the ownership of the value, simply use a reference to it: &str
fn reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Reference &str
    println!("The legth of '{s1}' is {len}");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

// To have the possibility to change a reference you have to use a mutable one
fn mutable_reference() {
    let mut s = String::from("hello");
    println!("String before change: {s}");
    change(&mut s);
    println!("String after change: {s}");
}
fn change(string: &mut String) {
    string.push_str(", world");
}
