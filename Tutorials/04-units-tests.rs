pub fn add(num: u32) -> u32 {
    num + 5
}

pub fn subtract(num: u32) -> u32 {
    num - 5
}

pub fn multiply(num: u32) -> u32 {
    num * 5
}

pub fn divide(num: u32) -> u32 {
    num / 5
}

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