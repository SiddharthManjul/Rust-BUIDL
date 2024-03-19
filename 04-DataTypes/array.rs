fn main() {
    let a = [1, 2, 3, 4, 5];

    // Accessing elements.
    let first = a[0];
    let second = a[1];

    println!("{}", first);
    println!("{}", second);
}

/*
Arrays can have only one type at a time, meaning every element in an array should be of same type.
Arrays are also used for grouping values.
Arrays are useful when you want your data allocated on the stack rather than the heap,
or when you want to ensure you always have a fixed number of elements. 
*/