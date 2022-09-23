use yew::{function_component, html, Callback, FocusEvent};

#[function_component(Login)]
pub fn login() -> Html {
    let log_in = {
        Callback::from(|e: FocusEvent| {
            e.prevent_default();
        })
    };

    html! {
        <form onsubmit={log_in}>
            <label for="nick">{"Nickname:"}</label>
            <input type="text" /><br />
            <input type="submit" value="Log in" />
        </form>
    }
}
