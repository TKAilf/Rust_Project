fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // もっと簡単にif letを使用することもできる
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let _coin = Coin::Quarter(State::Alabama);
    let _coin = Coin::Quarter(State::Alaska);
    let _coin = Coin::Dime;
    let _coin = Coin::Nickel;

    let mut _count = 0;
    match _coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => _count += 1,
    }
    let coin = Coin::Penny;
    // 上記のコードは以下のように書き換えることができる
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        _count += 1;
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}
