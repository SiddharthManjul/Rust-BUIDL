fn main() {
    let mut s = String::from("Hello");

    change(&mut s);

    println!("{s}");
}

fn change(mutable_string: &mut String) {
    mutable_string.push_str(", world!");
}

/*
First we change s to be mut. 
Then we create a mutable reference with &mut s where we call the change function,
and update the function signature to accept a mutable reference with some_string: &mut String. 
This makes it very clear that the change function will mutate the value it borrows.

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion
thus preventing the problem called Data Races.

We also cannot have mutable reference while we have immutable ones for the same value.
*/