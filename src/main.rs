use yew::{function_component, html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            <h1>{"Hello world"}</h1>
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
