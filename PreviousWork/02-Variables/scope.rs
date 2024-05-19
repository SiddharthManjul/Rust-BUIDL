fn main() {
    let x : i32 = 10;
    let y : i32 = 20;
    {
        let y : i32 = 5;
        println!("Value of x is {} & value of y is {}", x, y);
    }
    println!("Value of x is {} & value of y is {}", x, y);
}