fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value in cents: {}", value_in_cents(coin));
    let coin = Coin::Penny;
    println!("Value in cents: {}", value_in_cents(coin));
    let coin = Coin::Nickel;
    println!("Value in cents: {}", value_in_cents(coin));
    let coin = Coin::Dime;
    println!("Value in cents: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five: {:?}", five);
    println!("Six: {:?}", six);
    println!("None: {:?}", none);

    let some_u8_value = 0u8;
    some_value(some_u8_value);
}

#[derive(Debug)]
enum UsState {
    // Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

// Option<T>を使用したmatch式
// 必ずSomeとNoneの2つのケースを網羅する必要がある
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

// _というプレースホルダー
// 他のケースにマッチしない場合に使用する
// switch文でいうdefaultに相当するもの
fn some_value(some_u8_value: u8) -> (){
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

