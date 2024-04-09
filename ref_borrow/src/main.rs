fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // 以下のコードはコンパイルエラーになる。
    // 特定のデータに対して、特定のスコープ内で可変な参照を複数持つことはできない。
    // let r1 = &mut s;
    // let r2 = &mut s;

    // 以下のようにすればコンパイルエラーにならない。
    // r1 がスコープ外になりdrop関数が呼ばれるため、r2 がスコープ内で可変な参照を持つことができる。
    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;

    // 以下のコードはコンパイルエラーになる。
    // 参照と可変な参照を同時に持つことはできない。
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
}

// 関数の引数に参照を取ることを借用と呼ぶ。
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 借用した変数を変更することはできない。
// 以下の関数はコンパイルエラーになる。
// fn not_calculate_length(s: &String) {
//     s.push_str(" world");
// }

// 借用した変数を変更するためには、可変な参照を取る必要がある。
// mut をつけることで可変な参照を取ることができる。
fn change(s: &mut String) {
    s.push_str(" world");
}
