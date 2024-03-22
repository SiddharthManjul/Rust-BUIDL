fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String.
    s.len() 
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

// & -> Referencing Operator.
// * -> Dereferencing Operator.

/*
When functions have references as parameters instead of the actual values,
we won’t need to return the values in order to give back ownership, because we never had ownership.

We call the action of creating a reference borrowing. As in real life, if a person owns something,
you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

References are also immutable and thus we are not allowed to modify something we have a reference too.
*/