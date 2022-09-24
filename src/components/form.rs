use html::{onchange, onsubmit};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_submit: Callback<String>,
    pub error_msg: String,
}

#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    let handle_submit = props.handle_submit.clone();
    // state
    let input_text = use_state(|| String::default());
    let is_error = use_state(|| false);
    // shadows
    let input_text_shadow = input_text.clone();
    let is_error_shadow = is_error.clone();

    let onsubmit = Callback::from(move |e: onsubmit::Event| {
        e.prevent_default();
        if input_text.is_empty() {
            is_error.set(true);
        } else {
            handle_submit.emit(input_text.to_string())
        }
    });

    let onchange = Callback::from(move |e: onchange::Event| {
        let value = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        input_text_shadow.set(value);
    });

    html! {
        <form {onsubmit}>
            <input {onchange} type="text" /><br />
            if *is_error_shadow && !props.error_msg.is_empty() {
                <p>{props.error_msg.as_str()}</p>
            }
            <input type="submit" value="Send" />
        </form>
    }
}
