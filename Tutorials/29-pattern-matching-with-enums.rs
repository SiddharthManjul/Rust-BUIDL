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
}