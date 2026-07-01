struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    {
        // Basic method
        let width1 = 30;
        let height1 = 50;

        println!("=== BASIC METHOD ===");
        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }
    {
        // Tuple method
        let rect1 = (30, 50);

        println!("=== TUPLE METHOD ===");
        println!(
            "The area of the rectangle is {} square pixels.",
            tuple_area(rect1)
        );
    }
    {
        // Struct method
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("=== STRUCT METHOD ===");
        println!(
            "The area of the rectangle is {} square pixels.",
            struct_area(&rect1)
        );
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Refactoring with Tuples
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with struct
fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
