fn main() {

    // Dereferencing Coercion: Rust under the hood dereference the pointer for us and prints the value without using * and that's the reason why it directly prints out the value.
    // To print the address we need {:p}.
    // println!("{}", t); is same as println!("{}", *t); in the case below.
    let mut s: String = String::from("Hello String");
    let t: &mut String = &mut s;
    println!("{}", t);

    // Below example shows where and when do we need dereferencing.
    // Since we can't borrow from a String, we need to dereference it to make a pointer reference.
    let mut name: String = String::from("John");
    let name_t: &mut String = &mut name;
    *name_t = String::from("Shaun");
    println!("{}", name_t);

    // Another example
    let mut x: i32 = 50;
    x = 70;
    dbg!(x); // dbg!() is another way to print out the values and it will also tell the file and line number in the terminal.

    let y: &mut i32 = &mut x;
    *y += 1;
    dbg!(y);
    dbg!(x);
}

//{:p} is used to print the pointing address.