fn main() {
    let x : i32 = 5;
    let mut _y = 5;

    _y = x;

    let _v : u16 = 38_u8 as u16; /* The type annotation of v is u16, which matches the desired type. The expression 38_u8 as u16 performs a type cast,
    converting the u8 value 38 to a u16 value. This resolves the error and allows the code to compile and run without any issues. */

    let _z : i32 = 10;
    println!("Success");
}