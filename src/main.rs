mod app;
mod components;
mod context;
use crate::app::App;
use context::{socket::Socket, user::User};
use yew::{function_component, html, ContextProvider};

#[function_component(UniversalProvider)]
fn universal_provider() -> Html {
    let user_ctx = User::new();
    let ws_ctx = Socket::new();

    html! {
        <ContextProvider<Socket> context={ws_ctx}>
            <ContextProvider<User> context={user_ctx}>
                <App />
            </ContextProvider<User>>
        </ContextProvider<Socket>>
    }
}

fn main() {
    yew::start_app::<UniversalProvider>();
}
