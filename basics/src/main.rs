fn main() {
    
    // Closures
    let num: i32 = 5;
    let add_num = |x: i32| x + num;
    let new_num = add_num(7);
    dbg!(new_num);

    // Number Literals
    println!("Big number is {}", 98_222_000);
    println!("Hex is {}", 0xff);
    println!("Octal is {}", 0o77);
    println!("Binary is {}", 0b1111_0000);
    println!("Bytes 'A' is {}", b'A');

    // Raw - String Literal
    let text: &str = r#"{"Message" : "Rust is Awesome"}"#;
    dbg!(text);
}
