fn main() {
    let x : i32 = 5;
    let _y : i32; // Declaring variable with underscore doesn't throw warning when the varaible is unused.

    assert_eq!(x, 5); 
    /* assert_eq macro stands for assert equality which is used to assert that two expressions are equal to each other using the PartialEq trait. 
    If the expressions are not equal, the macro will panic and print the values of the expressions with their debug representations */
    println!("Success!")
}