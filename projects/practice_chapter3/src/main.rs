fn main() {
    println!("Temperature convertion: {}", fahrenheit_convertion(40));
    println!("Fibonacci Generator: {}", fibonacci_generator(4));
    outer_breaking();
}

fn fahrenheit_convertion(temperature: i32) -> i32 {
    let convertion = (temperature - 32) * (5 / 9);
    convertion
}

fn fibonacci_generator(nth_number: i32) -> i32 {
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let mut temp: i32;
    for _ in 1..nth_number {
        temp = b;
        b = a + temp;
        a = temp;
    }
    a
}

fn outer_breaking() {
    'outer_loop: loop {
        println!("Entering the inner inner loop!!");
        for i in 1..10 {
            println!("I'm at the inner loop for the {i} time");
            if i == 3 {
                println!("I'm too dizzy to stay in any loop!");
                break 'outer_loop;
            }
        }
    }
    println!("That's all folks!");
}
