#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_hashmap() {

        let person_1 = "alice";
        let person_2 = "bob";

        // Hashmap works on the basis of key value pair.
        // &str -> Person
        // u8 -> &str
        // &str -> u32

        let mut result_hashmap: HashMap<&str, u32> = HashMap::new();
        result_hashmap.insert(&person_1, 51);
        result_hashmap.insert(&person_2, 54);

        let test_score_1: Option<&u32> = result_hashmap.get(person_1);
        let test_score_2 = result_hashmap.get(person_2);

        if test_score_1 > test_score_2 {
            println!("Alice won with the score {:?}", test_score_1);
        } else {
            println!("Bob won with the score {:?}", test_score_2);
        }

    }
}