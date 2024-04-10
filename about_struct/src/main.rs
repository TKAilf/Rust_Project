fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     active: user1.active,
    //     sign_in_count: user1.sign_in_count,
    // };

    // 構造体更新記法
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // 残りのフィールドはuser1と同じ値にする
    };
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
