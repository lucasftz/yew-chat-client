mod modules;
use modules::hooks::use_auth::use_auth;
use modules::types::user::User;
use yew::{function_component, html};

#[function_component(App)]
fn app() -> Html {
    let user = use_auth();

    html! {
        <main>
            if User::is_authenticated(&user) {
                <h1>{user.unwrap().name}{" is signed in"}</h1>
            } else {
                <h1>{"User is not signed in"}</h1>
            }
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
