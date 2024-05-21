const MY_INTEGER: u8 = 10; // Constants in Rust should be declared using uppercase with underscore.


fn main() {

    // Stack
    let x: u8 = 50;
    println!("x is {}", x);

    // Heap
    let mut arr: Vec<u8> = vec![1, 2, 3, 4, 5];
    arr.push(10);
    println!("arr is {:?}", arr);

    // A reference on the Stack pointing to a value on the Heap.
    let arr_2 = &arr[0..3];
    println!("arr_2 is {:?}", arr_2);

    // Heap
    let mut s: String = String::from("Siddharth Manjul");
    s.push(' ');
    s.push('!');
    println!("s is {}", s);

    // A reference on the Stack pointing to a value on the Heap.
    let s_2 = &s[0..9];
    println!("s_2 is {}", s_2);

    println!("MY_INTEGER is {}", MY_INTEGER);
}
