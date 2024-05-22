#[derive(Debug)]

enum CarColour {
    Red,
    Green,
    Blue,
    Silver
}

fn create_car_colour() -> CarColour {
    let car_colour = CarColour::Blue;
    car_colour
}

// <T, E> - Generic Type or Generics
#[derive(Debug)]
enum GivenResult<T, E> { 
    Ok(T),
    Err(E)
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not Under 5!".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour = create_car_colour();
        dbg!(car_colour);

        let is_number_under_five = check_under_five(2);
        dbg!(is_number_under_five);

        let is_number_under_five = check_under_five(7);
        dbg!(is_number_under_five);
    }
}