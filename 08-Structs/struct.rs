// Defining a struct.

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Creating an instance of the User struct
    // Immutable.
    let user1 = User {
        active: true,
        username: String::from("Brooklyn"),
        email: String::from("Brooklyn@brooklyn.com"),
        sign_in_count: 1,
    };

    // Mutable.
    let mut user2 = User {
        active: true,
        username: String::from("Dynamis"),
        email: String::from("Darknight@darknight.com"),
        sign_in_count: 2,
    };

    // Changing email of user2
    user2.email = String::from("Dynamis@dynamis.com");

    println!("For user1: \nActive: {} \nName: {} \nEmail: {} \nSign In Count: {}", user1.active, user1.username, user1.email, user1.sign_in_count);
    println!("For user2: \nActive: {} \nName: {} \nEmail: {} \nSign In Count: {}", user2.active, user2.username, user2.email, user2.sign_in_count);
}