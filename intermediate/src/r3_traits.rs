#[allow(dead_code)]
trait Attacker {
    fn choose_name(&self) -> String;
    fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
#[allow(dead_code)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_weapon(&self) -> String {
        match self {
            Character::Warrior => "Excalibur".to_string(),
            Character::Archer => "Sarang Bow".to_string(),
            Character::Wizard => "Expecto Patronus".to_string(),
        }
    }

    fn choose_name(&self) -> String {
        match self {
            Character::Warrior => "King Arthur".to_string(),
            Character::Archer => "Arjuna".to_string(),
            Character::Wizard => "Albus Dumbledore".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn tests_traits() {
        let my_character = Character::Warrior;
        let chosen_name = my_character.choose_name();
        let chosen_weapon = my_character.choose_weapon();
        dbg!(chosen_name);
        dbg!(chosen_weapon);
    }
}
