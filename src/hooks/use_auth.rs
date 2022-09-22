use crate::types::user::User;

pub fn use_auth() -> Option<User> {
    // if the user is not logged in, return None, else, return Some<User>
    // hardcoded value for now
    let user = User::new("Carolina De La Cuba");

    Some(user)
}
