#[cfg(test)]
mod test {
    #[allow(dead_code, unused_variables, unused_imports)]
    use super::*;

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
