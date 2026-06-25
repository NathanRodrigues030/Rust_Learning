fn main() {
    explaining_data_using_strings();
}

fn explaining_data_using_strings() {
    // Moving data, excluding the first reference at heap
    let mut s = String::from("hello");
    s = String::from("ola");
    println!("{s}, world!");

    // In fact, cloning data from the heap
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}");
    println!("s2 = {s2}");
}
