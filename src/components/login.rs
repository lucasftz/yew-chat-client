use crate::html::{onchange, onsubmit};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handler: Callback<String>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let handler = props.handler.clone();
    // state
    let is_error = use_state(|| false);
    let text: UseStateHandle<Option<String>> = use_state(|| None);
    // shadows
    let is_error_shadow = is_error.clone();
    let text_shadow = text.clone();

    let submit = Callback::from(move |e: onsubmit::Event| {
        e.prevent_default();
        match &*text_shadow {
            Some(value) => handler.emit(value.to_owned()),
            None => is_error.set(true),
        }
    });

    let change = Callback::from(move |e: onchange::Event| {
        let value = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        text.set(Some(value));
    });

    html! {
        <form onsubmit={submit}>
            <label>{"Nickname:"}</label>
            <input type="text" onchange={change} /><br />
            if *is_error_shadow {
                <p>{"Please enter a valid nickname"}</p>
            }
            <input type="submit" value="Log in" />
        </form>
    }
}
