fn main() {
    let reference = dangle();

    println!("{}", reference);
}

// fn dangle() -> String {
//     let s = String::from("Hello from Dangle Function!");

//     &s // We return a reference to the String, s.
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

/*
Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
But we tried to return a reference to it. That means this reference would be pointing to an invalid String.
That’s no good! Rust won’t let us do this.
*/

fn dangle() -> String {
    let s = String::from("Hello from Dangle Function!");

    s
}

// Return the String directly. Don't use any reference to it.