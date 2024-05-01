struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("gun@gun.com"),
        username: String::from("gun"),
        sign_in_count: 1,
    };

    user1.username = String::from("donggun");

    let user2 = build_user(String::from("rust@rust.com"), String::from("dongun"));
    let user3 = User {
        active: false,
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}
