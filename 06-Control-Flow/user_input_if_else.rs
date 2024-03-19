use std::io;

fn main() {
    println!("Enter a number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input!");

    let number : i32 = input.trim().parse().expect("Invalid Number!");

    if number % 2 == 0 {
        println!("The number you entered is even!");
    } else {
        println!("The number you entered is odd!");
    }
}