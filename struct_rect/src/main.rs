fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_taple(rect1)
    );

    let rect2 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    // 構造体は出力方法がいくつもあるためprintln!マクロを使用して構造体自体を出力することはできない
    // 1や他の基本形をユーザーに表示するための方法は一つしかなく、それがprintln!マクロである
    // println!{"The rect2 is {}", rect2};
    // 構造体全体を表示するためには、デバッグフォーマットを使用する
    println!("The rect2 is {:#?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// タプルでリファクタリングを行う
fn area_taple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// タプルでリファクタリングを行うことで、引数が1つになり、可読性が向上する
// しかし、0や1といったインデックスを使うことで、どの値が何を表しているのかがわかりにくい
// そのため、構造体を使ってリファクタリングを行う
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
