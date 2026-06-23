fn main() {
    if_condition();
    if_in_a_let();
}

fn if_condition() {
    let number = 3;
    if number < 5 {
        println!("Lower than 5");
    } else if number > 10 {
        println!("Greater than 10");
    } else {
        println!("Ideal number");
    }
}

fn if_in_a_let() {
    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 10 };
    println!("The right number is: {number}");
}
