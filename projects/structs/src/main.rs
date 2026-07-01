struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Creating Instances with Struct Update Syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // .. means to have the same values of the fields in the given instance
    let user3 = User {
        username: String::from("exampleuser321"),
        email: String::from("another@example.com"),
        ..user1
    };

    user1.email = String::from("anotheremail@example.com");
}

// Field Init Shorthand syntax to when parameters names and the struct field names
// are excatly the same
fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
