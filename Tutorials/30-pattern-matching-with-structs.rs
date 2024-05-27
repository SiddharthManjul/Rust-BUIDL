#[cfg(test)]
mod test {
    #[allow(dead_code, unused_variables, unused_imports)]
    use super::*;

    #[test]
    fn test_match_gaurd() {
        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("They match!"),
            (x, y) if x + y == 0 => println!("They neutralize!"),
            (x, _) if x == 2 => println!("X is indeed +2!"),
            _ => println!("We are not bothered!"),
        };
    }

    #[allow(dead_code, unused_variables)]
    #[test]
    fn test_match_struct() {
        struct Location {
            x: i32,
            y: i32
        }
        let location = Location { x: 0, y: 20};
        match location {
            Location {x, y: 0} => println!("Y is on the axis!"),
            Location {x: 0, y} => println!("X is on the axis!"),
            Location {x, y} => println!("X and Y are not on the axis!"),
        };
    }
}
