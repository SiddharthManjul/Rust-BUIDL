extern crate rand;

use std::io; // io: Input/Output library. std is the standard library and io comes under it.
use rand::Rng; // Rng: Random number generation.
use std::cmp::Ordering; // cmp: compare two values.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=201);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read the line!");
        /* If we didn’t have the use std::io line at the beginning of the program, we could have written this function call as std::io::stdin. T
        he stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal. 
    
        References are immutable, that's why we have to write it as "&mut guess" instead of "&guess". Rust uses reference of data.
        */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Smaller!"),
            Ordering::Greater => println!("Greater!"),
            Ordering::Equal => {
                println!("That's Corrent, You win!");
                break;
        }
        }
    }
}
