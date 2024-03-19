fn main() {
    let a = multiplication(10, 9);

    println!("Product of given values is {}", a);
}

fn multiplication(x : i32, y : i32) -> i32 {
    x * y
}

// Expressions don't need semicolon.
// Semicolon is used only for indicating the end of an statement.