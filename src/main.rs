mod app;
mod components;
mod context;
use crate::app::App;
use context::user::User;
use yew::{function_component, html, ContextProvider};

#[function_component(UniversalProvider)]
fn universal_provider() -> Html {
    let ctx = User::new();

    html! {
        <ContextProvider<User> context={ctx}>
            <App />
        </ContextProvider<User>>
    }
}

fn main() {
    yew::start_app::<UniversalProvider>();
}
