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

    let user3 = User {
        email: String::from("yablubaablu@taablu.com"),
        ..user1 // Struct Update Syntax
    };

    // Changing email of user2
    user2.email = String::from("Dynamis@dynamis.com");

    // build_user function
    let user = build_user("Siddharth".to_string(), "jswabfciks@sdfvb.co".to_string());

    // Printing.
    // println!("For user1: \nActive: {} \nName: {} \nEmail: {} \nSign In Count: {}", user1.active, user1.username, user1.email, user1.sign_in_count);
    println!("For user2: \nActive: {} \nName: {} \nEmail: {} \nSign In Count: {}", user2.active, user2.username, user2.email, user2.sign_in_count);
    println!("For user3: \nActive: {} \nName: {} \nEmail: {} \nSign In Count: {}", user3.active, user3.username, user3.email, user3.sign_in_count);
    println!("For user: \nActive: {} \nName: {} \nEmail: {} \nSign In Count: {}", user.active, user.username, user.email, user.sign_in_count);
}

// A function which takes string as input and returns a User Struct Instance.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // or username: username (This is Field Init Shorthand)
        email, // or email: email (This is Field Init Shorthand)
        sign_in_count: 1,

    }
}

/*
We can no longer use user1 as we have borrowed username in user3 which will move data according to "Variable and Data Interacting with Move".
If we specified both email and username then user1 would still be valid after creating user3.
Both active and sign_in_count are types to implement the Copy trait, so the behaviour according to "Stack-Only Data: Copy" would apply.
*/