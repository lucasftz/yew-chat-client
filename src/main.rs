mod modules;
use modules::components::login::Login;
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
                <Login />
            }
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
