use yew::{function_component, html, Callback};

#[function_component(Login)]
pub fn login() -> Html {
    let log_in = { Callback::from(|_| ()) };

    html! {
        <button onclick={log_in}>{"Log in!"}</button>
    }
}
