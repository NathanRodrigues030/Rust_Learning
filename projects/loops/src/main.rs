fn main() {
    _for_range();
}

fn _infinite_loop() {
    // Enters in a infinite loop
    loop {
        println!("Again!");
    }
}

fn _using_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The loop result is: {result}");
}

// Really useful label { 'loop_label: } to control the outer loop from inside
fn _nested_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn _using_while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn _using_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("Value: {element}");
    }
}

fn _for_range() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
