mod funcs;
mod functions;
/// Letting the file know that there is a file called funcs.rs existing.
use crate::funcs::{add, subtract};
use crate::functions::other_funcs::divide;
/// Importing add & subtract functions from funcs. We can also use * for the same.
///  Everything in Rust is immutable by default.

fn main() {
    //To make a variable mutable we need to use mut keyword like this: let mut x: u32 = 50;
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y = add(x);
    println!("y is {}", y);

    x = 60;
    println!("x is {}", x);

    let z = subtract(x);
    println!("z is {}", z);

    let p = divide(x);
    println!("p is {}", p);
}
