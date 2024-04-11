fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1: {}, username: {}, active: {}, sign_in_count: {}", user1.email, user1.username, user1.active, user1.sign_in_count);

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

    println!("user2: {}", user2.email);

    build_user(String::from("emailemail"), String::from("usernameusername"));

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);

    let _unit = UnitStruct;

    // let user_lifetime = UserLifetime {
    //     username: "username",
    //     email: "email",
    //     active: true,
    //     sign_in_count: 1,
    // };
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

// タプル構造体
// タプル型に型名をつけたもので、同じ型のタプルを区別するために使われる
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// ユニット様構造体
// フィールドを持たない構造体
// 型にトレイトを実装するが、型自体に保持させるデータは一切ない場面に有効的な使用方法である。
struct UnitStruct;

// 上記の構造体においてString型を採用し&str型を採用していない
// これは、意図的なものであり構造体のインスタンスには全データを所有してもらう必要があるためである。
// つまり、構造体インスタンスのフィールドデータは構造体全体が有効な間は常に有効である必要があるということである。
// 構造体に参照&strを使用する場合にはライフタイムという機能を使用する必要がある。
// ライフタイムは、参照が有効である期間を示すものであり、コンパイラが参照が有効である期間を検証するために使用される。
// ライフタイムを実装していないため、以下のコードはエラーとなる。

// struct UserLifetime {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

