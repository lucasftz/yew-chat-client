use crate::components::form::Form;
use crate::context::socket::Socket;
use crate::User;
use yew::{function_component, html, use_context, Callback};

#[function_component(Login)]
pub fn login() -> Html {
    let user = use_context::<User>().unwrap();
    let mut ws = use_context::<Socket>().unwrap();

    let onsubmit = {
        ws.connect("wss://echo.websocket.events");
        Callback::from(move |nickname| user.log_in(nickname))
    };

    html! {
        <Form handle_submit={onsubmit} error_msg={"Please enter a valid nickname".to_string()} />
    }
}
