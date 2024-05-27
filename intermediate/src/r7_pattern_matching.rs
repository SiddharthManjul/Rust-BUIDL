#[allow(dead_code, unused_variables)]
#[derive(Debug)]

enum Message<'a> {
    Quit,
    ChangeColour(i32, i32, i32),
    Move {
        x: i32,
        y: i32
    },
    Write(&'a str)
}

#[allow(dead_code, unused_variables)]
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("I quit");
        },
        Message::ChangeColour(red, green, blue) => {
            println!("Red {}; Green {}; Blue {}", red, green, blue);
        },
        Message::Move { x, y: new_name } => {
            println!("X is {} and Y as new_name is {}", x, new_name);
        },
        Message::Write(text) => {
            println!("{}", text);
        }
    };
}

#[cfg(test)]
mod test {
    #[allow(dead_code, unused_variables, unused_imports)]
    use super::*;

    #[test]

    // Large Enums
    fn test_large_enums() {
        let quit = Message::Quit;
        process_message(quit);

        let colour = Message::ChangeColour(10, 20, 255);
        process_message(colour);

        let on_move = Message::Move { x: 30, y: 50 };
        process_message(on_move);

        let written = Message::Write("I love Rust");
        process_message(written);
    }

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
