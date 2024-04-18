// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// // useを使用すると、まるで登録したモジュールがcrateの一部であるかのように、そのモジュールの要素にアクセスできる
// // use crate::front_of_house::hosting;
// // 相対パスで書くことも可能
// use self::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// // 以下のようにすることでhosting::を省略できる
// // しかし、関数をスコープにuseで持ち込む際に、慣例的に関数の呼び出しにはcrate名をつけることが多い
// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
//     add_to_waitlist();
//     add_to_waitlist();
// }

// // 一方、構造体やenumその他の要素をuseで持ち込む際には、フルパスを書くのが慣例的である。
// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// // 以下のように同名の関数をuseで持ち込むことはできない
// // use std::fmt::Result;
// // use std::io::Result;

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     Ok(())
// }

// fn function2() -> io::Result<()> {
//     Ok(())
// }

// // 新しい名前をasキーワードで与える
// // 以下のようにすることで、同名の関数をuseで持ち込むことができる
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     Ok(())
// }

// fn function2() -> IoResult<()> {
//     Ok(())
// }

// // pub useを使用して名前を再公開する
// // useキーワードで名前をスコープに持ち込むと、その名前はprivateになる
// // 再公開しないと、他のコードからその名前にアクセスできない
// // 


// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// // 巨大なuseのリストをネストしたパスを使って整理する。
// // use std::cmp::Ordering;
// // use std::io;
// // これらをまとめることができる
// use std::{cmp::Ordering, is};

// // use std::io;
// // use std::io::Write;
// // これらをまとめることができる
// use std::io::{self, Write};

// // glob(global)演算子を使用して、すべての公開要素を持ち込むこともできる
// // ただし、この方法はどこで定義されたのか分かりづらいため一般的には推奨されていない
// // テストコードなどで使用されるのが一般的な使用方法
// use std::collections::*;
