#[cfg(test)]
mod test {
    #[allow(dead_code, unused_variables, unused_imports)]
    use super::*;

    #[test]
    fn test_match_literals() {
        let number: i32 = 20;

        let res: &str = match number {
            1 => "This is the first!",
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 20 => "We found it in the sequence!",
            _ => "It was something else!",
        };

        println!("{}", res);
    }
}
