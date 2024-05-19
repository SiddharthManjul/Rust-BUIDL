extern crate rand; // Import the rand crate

use rand::Rng; // Import the Rng trait from the rand crate

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // Use the gen_range method from the Rng trait
    println!("Secret number: {}", secret_number);
}
