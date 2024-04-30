struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {

    // we must use mut keyword to make the any field mutable
    let mut user_one = User {
        active: true,
        username: String::from("johndoe"),
        email: String::from("johndoe@mail.com"),
        sign_in_count: 1,
    };

    user_one.email = String::from("updated@mail.com");

    let user_two = build_user("shelton@mail.com".to_string(), "shelton".to_string());

    let im_a_lazy_user = User {
        email: String::from("lazy@mail.com"),
        ..user_one
    };
}
