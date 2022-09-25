use yew::{use_state, UseStateHandle};

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    nick: UseStateHandle<String>,
    logged_in: UseStateHandle<bool>,
}

impl User {
    pub fn new() -> Self {
        User {
            nick: use_state(|| String::default()),
            logged_in: use_state(|| bool::default()),
        }
    }

    pub fn log_in(&self, nick: String) {
        self.nick.set(nick);
        self.logged_in.set(true);
    }

    pub fn log_out(&self) {
        self.nick.set(String::default());
        self.logged_in.set(false);
    }

    pub fn nick(&self) -> String {
        self.nick.to_string()
    }

    pub fn logged_in(&self) -> bool {
        *self.logged_in
    }
}
