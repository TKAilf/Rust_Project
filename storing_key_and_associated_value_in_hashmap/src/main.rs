use std::collections::HashMap;
fn main() {
    // new():新規ハッシュマップを生成する
    // insert():ハッシュマップにキーと値を追加する
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::fropm("Blue"), String::from("Yellow")];
    // let initial_scores = vec1[10, 50];
    // HashMap<_, _>型では_によって型推論ができる。
    // vector型の各要素をもとにHashMapを生成するため、型推論が可能となる
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // ハッシュマップはCopyトレイトを実装する型について、値はハッシュマップにコピーされる。
    // Stringのような所有権を持つ値なら、所有権がムーブされる

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // field_nameとfield_valueはこの時点で無効化されるため
    // これ以降使用することはできない

    // ハッシュマップの値にアクセスする
    // get():ハッシュマップの値を取得する
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let scores_value = scores.get(&team_name);
    println!("{:?}", scores_value);
    match scores_value {
        Some(value) => println!("The value is {}", value),
        None => println!("The value is not found"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // ハッシュマップを更新する
    // 値を上書きする
    // 同じキーを異なる値でinsert()を使用し挿入すると置換される。
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // キーに値がない場合のみ挿入する
    // entry()メソッドを使用することで、キーに値がない場合のみ挿入することができる
    // entry():ハッシュマップにキーが存在するかどうかを確認する
    // 戻り値はEntryと呼ばれるenum
    // or_insert():Entryのメソッドで、キーが存在しない場合に値を挿入する
    let mut scores_without_key = HashMap::new();
    scores_without_key.insert(String::from("Blue"), 10);

    scores_without_key.entry(String::from("Yellow")).or_insert(50);
    scores_without_key.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores_without_key);

    // 古い値に基づいて値を更新する
    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", text_map);
}
