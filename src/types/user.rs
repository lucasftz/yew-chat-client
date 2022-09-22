pub struct User {
    pub name: String,
    pub logged_in: bool,
}

impl User {
    pub fn new(name: &str) -> User {
        return User {
            name: String::from(name),
            logged_in: true,
        };
    }

    pub fn is_authenticated(possible_user: &Option<User>) -> bool {
        return match possible_user {
            Some(_) => true,
            None => false,
        };
    }
}
