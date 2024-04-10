fn main() {
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear();

    // println!("The first word is: {}", word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    // let hello = &s[..5]; // 0 から始まる場合は省略可能
    let world = &s[6..11];
    // let world = &s[6..]; // 11 までの場合は省略可能
    let slice = &s[..];
    let word = first_word(&s);
    println!("hello:{}, world:{}, slice: {}, word: {}", hello, world, slice, word);

    let my_string = String::from("hello world");
    let word2 = first_word_aprove(&my_string[..]);
    let my_string_literal = "hello world";
    let word3 = first_word_aprove(&my_string_literal[..]);
    let word4 = first_word_aprove(my_string_literal);
    println!("word2: {}, word3: {}, word4: {}", word2, word3, word4);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// 以下の関数シグニチャにすることで、&String と &str の両方を引数に取ることができる。
// 仮にStrinに対して扱いたいときは、String全体のスライスを取ることで対応できる。
// 例えば、&s[..] とすることで、String 全体のスライスを取得できる。
// また、文字列リテラルに対して扱いたいときは、文字列リテラルのスライスを取ることで対応できる。
// しかし、文字列リテラル全体を扱いたいときは、文字列リテラルそのものを引数に取ることで対応できる。
// なぜなら、文字列リテラルはそれ自体がすでに文字列スライスであるためである。
fn first_word_aprove(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
