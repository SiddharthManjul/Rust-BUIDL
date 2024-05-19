fn main() {
    let tuple : (i32, f32, u8) = (500, 7.8, 45);

    // Destructuring through pattern matching.
    let (x, y, z) = tuple;

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);

    let a : (i64, f32, u16) = (123456, 9876.54, 3645);

    // Destructuring elements by accessing them using period(.).
    let b = a.0;
    let c = a.1;
    let d = a.2;

    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}

/*
Tuples can have different types in a single tuple. 
Tuples are used for grouping values.
*/