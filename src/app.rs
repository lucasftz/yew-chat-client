use crate::components::chatroom::ChatRoom;
use crate::components::login::Login;
use crate::User;
use yew::{function_component, html, use_context, Callback};

#[function_component(App)]
pub fn app() -> Html {
    let user = use_context::<User>().unwrap();
    let user_shadow = user.clone();

    let handle_login = Callback::from(move |text| user.log_in(text));

    html! {
        <main>
            if user_shadow.logged_in() {
                <ChatRoom />
            } else {
                <Login handler={handle_login} />
            }
        </main>
    }
}
