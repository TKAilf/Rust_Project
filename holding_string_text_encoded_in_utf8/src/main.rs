fn main() {
    // // String型のインスタンスを生成する
    // let mut s = String::new();

    // // 文字列リテラルからString型のインスタンスを生成する
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();

    // // String::from関数を使ってString型のインスタンスを生成する
    // // 上記のto_stringメソッドと同じことを行う
    // let s = String::from("initial contents");

    // // 文字列を更新する
    // let mut s = String::from("foo");
    // s.push_str("bar");

    // // push_strメソッドは引数の所有権を奪わない
    // // push_strメソッドは参照を受け取る
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {}", s2);

    // // pushメソッドは1文字を追加する
    // let mut s = String::from("lo");
    // s.push('l');

    // +演算子, またはformat!マクロで文字列を連結する
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // s1はムーブされているので、この時点で使用できない
    // 上記のコードをよく考えてみると
    // &s2は&str型であり、&String型ではない
    // これはコンパイラが&String型を&str型に自動的に変換しているためである
    // これを型強制と呼ぶ
    // またaddメソッド呼び出しの際に、コンパイラは参照外し型強制というものを使用し
    // &s2を&s2[..]に変換している

    // 複数の文字列を連結する必要が出ると、+演算子は使いづらくなる
    // let s4 = String::from("tic");
    // let s5 = String::from("tac");
    // let s6 = String::from("toe");
    // let s7 = s4 + "-" + &s5 + "-" + &s6;
    // println!("s7 is {}", s7);

    // 上記ではわかりづらいので、format!マクロを使用する
    // let sTic = String::from("tic");
    // let sTac = String::from("tac");
    // let sToe = String::from("toe");
    // let sToe = format!("{}-{}-{}", sTic, sTac, sToe);
    // format!マクロはprintln!マクロと同じように動作する

    // 文字列に添え字アクセスする
    // let sstr = String::from("Здравствуйте");
    // let sAns = &sstr[0..4];
    // let sAns = &sstr[0..1]; // これはエラーになる
    // なぜなら、&sstr[0..1]は1バイト目の文字を取得しようとしているが、
    // 1バイト目の文字は1バイトでは表現できない文字であるためである
    // Зは2バイトで表現される文字である
    // println!("sAns is {}", sAns);

    // 文字列を走査する
    // 幸いにも、Unicodeスカラー値に対して処理を行う場合は、
    // 最適解としてcharsメソッドを使用することができる
    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }
    // // bytesメソッドを使用して、各バイトをそのまま返すこともできる
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

