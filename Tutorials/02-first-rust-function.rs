fn add_five(num: u32) -> u32 {
    num + 5
}

fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y = add_five(x);
    println!("y is {}", y);

    x = 60;
    println!("x is {}", x);
}

// Everything in Rust is immutable by default.
// To make a variable mutable we need to use mut keyword like this: let mut x: u32 = 50;