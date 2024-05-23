#[derive(Debug)]
#[allow(dead_code)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let user1 = User {
            username: String::from("someuser1"),
            email: String::from("user1@example.com"),
            sign_in_count: 1,
            is_active: true,
        };
        dbg!(user1);
    }
}

/*
Enums can hold structs and structs can hold enums.
*/