fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

enum IpAddrKind {
    V4,
    V6,
}

// 構造体のようにenumの各要素にデータを持たせることもできる。
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

// このようにenumの各要素に異なる型のデータを持たせることもできる。
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 標準ライブラリにIPアドレスを表す方がある。以下確認してみる。
// 以下のようにenumの各要素にstructを持たせることで、いかなるデータ構造も表現できるようになっている。
// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// 以下のようにenumにメソッドを実装することもできる。
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 構造体のようにデータを持たせることもできる。
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 以下のようにstructを使っても同じことができるが、enumの方が簡潔に書ける。
// structとenumはどちらも酷似しているが、enumは異なる型のデータを持たせることができる。
struct QuitMessage;
struct MoveMessage{
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// もう一点、似通っているものとしてenumにもメソッドを実装できる。
impl Message {
    fn call(&self) {
        println!("callメソッドを実行しました。");
    }
}

// 一般的で有用なenumの使い方としてOption<T>がある。
// Option<T>の列挙子を使用することで、null値を扱うことができる。
// Option<T>により、null参照によるエラーを回避することができる。
// Option<T>にはSome(T)とNoneの2つの列挙子がある。
// Some(T)は値を持ち、Noneは値を持たない。
// 以下はOption<T>のドキュメントである。
// https://doc.rust-lang.org/std/option/enum.Option.html
