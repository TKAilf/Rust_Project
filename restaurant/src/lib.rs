mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// superを相対パスで使用すると、親モジュールにアクセスできる
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order(); // 絶対パスではこのように書く
    }

    fn cook_order() {}

    // 構造体自体にpubをつけることで、その構造体は公開されるが、構造体のフィールドは公開されない
    // 構造体のフィールドにpubをつけることで、そのフィールドは公開される
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enum自体にpubをつけることで、enumの各要素も公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// eat_at_restaurant関数からadd_to_waitlist関数を呼び出す方法
pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn main() {}

// パスには2つの種類がある。絶対パスと相対パスである。
// 絶対パスは、クレートの名前かcrateという文字列から始める。
// 相対パスは、現在のモジュール内の識別子か、self、superという文字列から始める。
//
// add_to_waitlistのパスは何でしょうか？

