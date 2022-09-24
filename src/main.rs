mod modules;
use gloo::console::log;
use modules::components::login::Login;
use yew::{function_component, html, Callback};

#[function_component(App)]
fn app() -> Html {
    let is_logged_in = false; // hard coded for now

    let handle_login = Callback::from(|input| {
        log!(input);
    });

    html! {
            <main>
            if is_logged_in {
                <h1>{"User is signed in"}</h1>
            } else {
                <Login handler={handle_login} />
            }
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
