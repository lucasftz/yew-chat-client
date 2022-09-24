use crate::User;
use yew::{function_component, html, use_context, Callback};

#[function_component(ChatRoom)]
pub fn chatroom() -> Html {
    let user = use_context::<User>().unwrap();
    let user_shadow = user.clone();

    let logout = Callback::from(move |_| user.log_out());

    html! {
        <>
            <p>{"Username: "}{user_shadow.nick()}</p>
            <button onclick={logout}>{"Log out"}</button>
            <form>
                <input type="text" />
                <input type="submit" value="Send" />
            </form>
        </>
    }
}
