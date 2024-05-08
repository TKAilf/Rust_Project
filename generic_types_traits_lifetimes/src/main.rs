fn main() {
    // 関数を抽出することで重複を取り除く
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
    println!("float.x = {}, float.y = {}", float.x, float.y);
    println!("integer.x = {}", integer.x());
    let wont_work = Point2 { x: 5, y: 4.0 };
    println!("wont_work.x = {}, wont_work.y = {}", wont_work.x, wont_work.y);
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ジェネリック型を使って関数を抽出する
// これだけではstd::cmp::PartialOrdトレイトを実装していないためコンパイルエラー
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// 構造体を使ってジェネリック型を実装する
struct Point<T> {
    x: T,
    y: T,
}

// メソッド定義にジェネリック型を使う
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// ジェネリック型はコンパイル時に具体的な型に置き換えられるため
// xとyの型が異なるとコンパイルエラーになるため、以下のようにする
// 複数のジェネリック型を実装する
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
