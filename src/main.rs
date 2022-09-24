mod app;
mod components;
use crate::app::App;
use yew::{function_component, html, use_state, ContextProvider};

#[derive(Clone, Debug, PartialEq)]
struct User {
    nick: Option<String>,
    logged_in: bool,
}

#[function_component(UniversalProvider)]
fn universal_provider() -> Html {
    let ctx = use_state(|| User {
        nick: None,
        logged_in: false,
    });

    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
            <App />
        </ContextProvider<User>>
    }
}

fn main() {
    yew::start_app::<UniversalProvider>();
}
