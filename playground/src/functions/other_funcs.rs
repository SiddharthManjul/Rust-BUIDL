/// # Function: multiply 
/// 
/// # Argument: (num: u32)
/// 
/// # Example:
/// ```
/// let x = 5;
/// let p = add(x);
/// ```

pub fn multiply(num: u32) -> u32 {
    num * 5
}

/// # Function: divide 
/// 
/// # Argument: (num: u32)
/// 
/// # Example:
/// ```
/// let x = 5;
/// let q = add(x);
/// ```

pub fn divide(num: u32) -> u32 {
    num / 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiplies_a_number_test() {
        let x = 4;
        let y = multiply(x);
        println!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 20);
    }

    #[test]
    fn divides_a_number_test() {
        let x = 50;
        let y = divide(x);
        println!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 10);
    }
}
