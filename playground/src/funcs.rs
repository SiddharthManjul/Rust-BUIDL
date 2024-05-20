pub fn add(num: u32) -> u32 {
    num + 5
}

pub fn subtract(num: u32) -> u32 {
    num - 5
}

// To run a test we have to use cargo test command
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

// pub keyword is used to make a function public.