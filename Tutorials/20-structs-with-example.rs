#[derive(Debug)]
#[allow(dead_code)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool
}

impl User {
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {

        let mut user1 = User {
            username: String::from("someuser1"),
            email: String::from("user1@example.com"),
            sign_in_count: 1,
            is_active: true,
        };

        // user1.username = "anotherusername".to_string(); 
        change_username(&mut user1, "somenewname");

        dbg!(user1);

        let mut user2 = User {
            username: String::from("someuser2"),
            email: String::from("user2@example.com"),
            sign_in_count: 10,
            is_active: false,
        };

        user2.increment_sign_in_count();
        user2.change_email("user2newexample.com");

        dbg!(user2);
    }
}

/*
Enums can hold structs and structs can hold enums.
*/