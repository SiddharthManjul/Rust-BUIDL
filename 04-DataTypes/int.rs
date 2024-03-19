fn main() {
    let x : i64 = 123456;
    println!("x is declared as i64 and the value is {}", x);

    let mut y : i8 = 2;
    println!("y is declared as i8 and the value is {}", y);

    y = 4;
    println!("Since y is declared as mutable variable, the changed value is {}", y);
}

/* 
Int is scalar type.
Both signed and unsigned integers are available in rust. For signed we use i whereas for unsigned we use u.
Signed Integers: i8, i16, i32 and i64.
Unsigned Integers: u8, u16, u32, u64.
For 8, 16, 32 and 64-bit systems.

Signed variant can store numbers from -(2^(n - 1)) to 2^(n - 1) - 1.
Unsigned variant can store number from 0 to 2^n - 1.

Rust used i32 as default type.
*/