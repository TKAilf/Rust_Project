fn main() {
    // let v: Vec<i32> = Vec::new();
    // より一般的には、vec! マクロを使う
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // ベクタがdropすると、要素もdropされる
    // 要素に対する参照を使い始めると、少し複雑になってくる。
    // 以下に例を示す。

    // ベクタの要素を読む
    // ベクタの要素を読む方法は、2種類ある。
    // 添え字記法とgetメソッドを使う方法である。
    let v = vec![1, 2, 3, 4, 5];

    // 添え字記法
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // getメソッド
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
