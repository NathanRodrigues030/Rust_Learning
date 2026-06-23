fn main() {
    another_function(5, 'H');
    expressions();
    println!("{}", number_returning());
}

fn another_function(x: i32, letter: char) {
    println!("The value of x is: {x}, {letter}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn number_returning() -> u8 {
    11
}
