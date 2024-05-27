#[cfg(test)]
mod test {
    #[allow(dead_code, unused_variables, unused_imports)]
    use super::*;

    // Integers
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

    // Options
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

    #[test]
    fn test_match_results() {
        let some_res: Result<i32, &str> = Ok(10);
        // let some_err: Result<i32, &str> = Err("Program Terminated!");

        let res = match some_res {
            Ok(i) => i,
            Err(e) => panic!("{}", e)
            
        };
        println!("{}", res);
        
    }
}
