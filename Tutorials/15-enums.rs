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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour = create_car_colour();
        dbg!(car_colour);
    }
}