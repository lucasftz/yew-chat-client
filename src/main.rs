mod components;
use components::login::Login;
use yew::{function_component, html, use_state, Callback};

#[function_component(App)]
fn app() -> Html {
    // state
    let nickname = use_state(|| String::default());
    let is_logged_in = use_state(|| false);
    // handlers
    let nickname_handler = nickname.clone();
    let is_logged_in_handler = is_logged_in.clone();

    let handle_login = Callback::from(move |input| {
        nickname.set(input);
        is_logged_in.set(true);
    });

    html! {
            <main>
            if *is_logged_in_handler {
                <h1>{&*nickname_handler}{" is logged in"}</h1>
            } else {
                <Login handler={handle_login} />
            }
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
