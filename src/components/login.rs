use crate::components::form::Form;
use crate::User;
use yew::{function_component, html, use_context, Callback};

#[function_component(Login)]
pub fn login() -> Html {
    let user = use_context::<User>().unwrap();

    let onsubmit = Callback::from(move |nickname| user.log_in(nickname));

    html! {
        <Form handle_submit={onsubmit} error_msg={"Please enter a valid nickname".to_string()} />
    }
}
