fn main() {
    variable_type();
    scope();
    tuple();
    array_type();
}
fn variable_type() {
    // Example using boolean type
    let verify: bool = false;
    println!("{verify}");
}
fn scope() {
    // Variables Scope
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn tuple() {
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, z) = tup;
    let index2 = tup.2;
    println!("That's a tuple value: {z}, {index2}");
}

fn array_type() {
    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    let equal_array = [0; 5];
    println!("First Element: {}", numbers[0]);
    println!("Array building: {equal_array:?}");
}
