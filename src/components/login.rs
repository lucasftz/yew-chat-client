use crate::html::{onchange, onsubmit};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handler: Callback<String>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let handler = props.handler.clone();
    let text = use_state(|| String::default());
    let text_shadow = text.clone();

    let submit = Callback::from(move |e: onsubmit::Event| {
        e.prevent_default();
        handler.emit(text_shadow.to_string());
    });

    let change = Callback::from(move |e: onchange::Event| {
        let value = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        text.set(value);
    });

    html! {
        <form onsubmit={submit}>
            <label>{"Nickname:"}</label>
            <input type="text" onchange={change} /><br />
            <input type="submit" value="Log in" />
        </form>
    }
}
