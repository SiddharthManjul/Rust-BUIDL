/// # Function: add 
/// 
/// # Argument: (num: u32)
/// 
/// # Example:
/// ```
/// let x = 5;
/// let y = add(x);
/// ```

pub fn add(num: u32) -> u32 {
    num + 5
}

/// # Function: add 
/// 
/// # Argument: (num: u32)
/// 
/// # Example:
/// ```
/// let x = 5;
/// let z = subtract(x);
/// ```

pub fn subtract(num: u32) -> u32 {
    num - 5
}

/// pub keyword is used to make a function public.
/// Use "cargo doc" to create documentation from the comment in html format and to open it run command "cargo doc --open".

/// To run a test we have to use "cargo test" command.
/// To let the tests print out the values, use "cargo test -- --nocapture" command.
/// To run a single test put the test function name after cargo test: "cargo test adds_a_number_test"
/// In case a test fails, use "RUST_BACKTRACE=1 cargo test test_name" to check the issues.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_a_number_test() {
        let x = 100;
        let y = add(x);
        println!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 105);
    }

    #[test]
    fn subtract_a_number_test() {
        let x = 100;
        let y = subtract(x);
        println!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 95);
    }
}
