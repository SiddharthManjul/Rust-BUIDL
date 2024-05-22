fn main() {
    
    // Binary
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;
    println!("a's value is {}", a);
    println!("b's value is {}", b);

    println!("a's binary is {:08b}", a);
    println!("b's binary is {:08b}", b);

    // Logic Gates
    println!("AND {:08b}", a & b);
    println!("OR {:08b}", a | b);
    println!("XOR {:08b}", a ^ b);
    println!("NOT A {:08b}", !a);

    // Bitwise Operations
    println!("a << 1 {:08b}", a << 1);
    println!("a << 1 {}", a << 1);
    println!("a >> 1 {:08b}", a >> 1);
    println!("a >> 1 {}", a >> 1);
}
