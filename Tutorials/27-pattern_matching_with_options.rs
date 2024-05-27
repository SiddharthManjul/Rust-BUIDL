#[cfg(test)]
mod test {
    #[allow(dead_code, unused_variables, unused_imports)]
    use super::*;

    #[test]
    fn test_match_options() {
        let some_num: Option<i32> = Some(10);
        // let prob_none: Option<i32> = None; --- Using match statement on this one will go for None case only.

        let res = match some_num {
            Some(i) => i,
            None => {
                panic!("There was some Problem!");
            }
        };

        // Another way
        if let Some(i) = some_num {
            println!("From if statement, {}", i);
        } else {
            panic!("There was a Problem!");
        }

        println!("{:?}", some_num);
        println!("{}", res);
    }
}
