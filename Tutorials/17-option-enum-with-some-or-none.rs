use core::num;
#[allow(dead_code)]
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
#[allow(dead_code)]
enum GivenResult<T, E> { 
    Ok(T),
    Err(E)
}

// Some or None
#[derive(Debug)]
#[allow(dead_code)]
enum GivenOption<T> { 
    None,
    Some(T)
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not Under 5!".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
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

        let remainder = remainder_zero(12.2);
        dbg!(remainder);
    }
}