// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// default implementation
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// Add summarize_author
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
// Add summarize_author version
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }
// Add summarize_author version
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// trait bound(トレイト境界)
// 下記コードでいうトレイト境界は&impl Summaryの部分
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 複数のトレイト境界を指定することもできる
// + を使って指定する
// pub fn notify(item: &(impl Summary + Display)) {
// また、糖衣構文を使って下記のように書くこともできる
// pub fn notify<T: Summary + Display>(item: &T) {

// some_function<T: Display + Clone, U: Clone + Debug>(t: %T, u: &U) -> i32 {
// where句を使って、より読みやすい形にすることもできる
// pub fn some_function<T, U>(t: T, u: U) -> i32
// where T: Display + Clone,
//       U: Clone + Debug
// {

// トレイトを実装している型を返すこともできる
// fn returns_summarizable() -> impl Summary {
//     Tweet{
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }

// トレイト境界を使用して、メソッド実装を条件分けする
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 別のトレイトを実装するあらゆる型に対するトレイト実装を条件分けすることもできる。
// ブランケット実装：トレイト境界を満たすあらゆる型にトレイトを実装すること
// 例えば、標準ライブラリはDisplayトレイトを実装するあらゆる型にToStringトレイトを実装している。
// impl<T: Display> ToString for T {
//     // --snip--
// }
