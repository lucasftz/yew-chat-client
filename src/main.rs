mod app;
mod components;
use crate::app::App;
use yew::{function_component, html, use_state, ContextProvider, UseStateHandle};

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    nick: UseStateHandle<String>,
    logged_in: UseStateHandle<bool>,
}

impl User {
    fn log_in(&self, nick: String) {
        self.nick.set(nick);
        self.logged_in.set(true);
    }

    fn nick(&self) -> String {
        self.nick.to_string()
    }

    fn logged_in(&self) -> bool {
        *self.logged_in
    }
}

#[function_component(UniversalProvider)]
fn universal_provider() -> Html {
    let ctx = User {
        nick: use_state(|| String::default()),
        logged_in: use_state(|| false),
    };

    html! {
        <ContextProvider<User> context={ctx}>
            <App />
        </ContextProvider<User>>
    }
}

fn main() {
    yew::start_app::<UniversalProvider>();
}
