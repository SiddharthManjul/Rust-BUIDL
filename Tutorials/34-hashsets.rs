#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn tests_hashset() {

        // Instead of storing key value pairs, we can use Hashsets for storing single value either "key" or "value".

        let mut names_hashset = HashSet::new();
        names_hashset.insert("Alice");
        names_hashset.insert("Bob");
        names_hashset.insert("Jane");

        if names_hashset.contains("Bob") {
            dbg!("Bob is present!");
        }
    }
}