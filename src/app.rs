use crate::components::chatroom::ChatRoom;
use crate::components::login::Login;
use crate::User;
use yew::{function_component, html, use_context};

#[function_component(App)]
pub fn app() -> Html {
    let user = use_context::<User>().unwrap();

    html! {
        <main>
            if user.logged_in() {
                <ChatRoom />
            } else {
                <Login />
            }
        </main>
    }
}
