fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        email, // 仮引数名と構造体のフィールド名が同じ時は省略できる
        // username: username,
        username, // 仮引数名と構造体のフィールド名が同じ時は省略できる
        active: true,
        sign_in_count: 1,
    }
}
