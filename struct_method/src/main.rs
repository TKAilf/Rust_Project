fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("The area of the square is {} square pixels.", sq.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッドと関数の違い
// メソッドは構造体や列挙型のインスタンスに対して呼び出されるもの
// 最初の引数は必ずselfとなる
// impl(implementation; 実装)ブロック内にメソッドを定義する
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 関連関数
    // implブロック内でselfを引数に取らない関数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
