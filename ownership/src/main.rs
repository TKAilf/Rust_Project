fn main() {
    let s1 = "hello";
    println!("{}", s1);

    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("{}", s2);
    // s1とs2はどちらも文字列を保持しているが、
    // s1は文字列リテラルを、s2はString型の値を保持しているという違いがある。
    // これらはメモリ上ではスタックとヒープという場所に保存されている。
    // 一定の大きさを持っているリテラルやint, floatなどの値はスタックに保存される。
    // 一方で、可変長の文字列などはヒープに保存される。
    // ヒープに保持されたデータは、そのスコープを外れた際にdrop関数が呼ばれてメモリが解放される。

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // 値5をxに束縛し、その後yにxの値を代入している。

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{}, world!", s4);
    // 参考：https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html
    // s3はs4に束縛された後にdrop関数が呼ばれるため、s3は使用できなくなる。
    // s3に含まれる"hello"はスタック上に保存されているが、
    // s3の文字列の中身を保持するメモリへのポインタ、長さ、許容量などの情報はヒープ上に保存されている。
    // s3に含まれるヒープ上のデータはs4に移動されるが、スタック上のデータはコピーされず同じものを使いまわすことになる。
    // このとき、s3とs4は同じヒープ上のデータを指しているため、s3のdrop関数が呼ばれるとメモリを二重開放してしまう。
    // この問題を解決するために、Rustはs3の所有権をs4に移動するときにs3を無効化する。
    // これは、Rustの所有権システムによって、メモリの二重解放やデータ競合を防ぐための仕組みである。
    // また、このようなヒープ上のデータのみをコピーする動作をshallow copyと呼ぶ。
    // しかし、Rustではコンパイラが最初の変数(s3)をも無効化するため
    // shallow copyではなく、moveと呼ぶ。
    // またヒープ上のデータだけでなくスタック上のデータまでもコピーする動作をdeep copyと呼ぶ。
    // Rustではdeep copyはcloneメソッドを使用して行う。

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // ここでもう一度上記のコードを見てみると、
    // 一見すると、今学んだことと矛盾しているようにみえるが、既知のサイズを持つ方はスタックに保存されるため、
    // cloneメソッドを使用しなくてもdeep copyが行われる。
    // またこの際のdeep copyとshallow copyに違いはなく一般的なshallow copy以上のことをしなくなる。

    // RustにはCopyトレイトと呼ばれる特別な注釈がある。
    // int, float, boolなどの既知のサイズを持つスタックに保持される方に対して配置することができる。
    // Copyトレイトに適合している場合、代入後も古い変数が使用可能になる。
    // ただし、型や一部分でもDropトレイトを実装している場合、Copyトレイトを実装することはできない。
    // つまり、参照型やヒープ上にデータを保持する型はCopyトレイトを実装することができない。
    // ちなみに、タプル型でもCopyトレイトを実装することができるが、その要素が全てCopyトレイトを実装している場合に限る。
    // 例えば、(i32, i32)はCopyトレイトを実装しているが、(i32, String)はCopyトレイトを実装していない。


    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫




    let _s5 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                        // ムーブする

    let _s6 = String::from("hello");     // s2がスコープに入る

    let _s7 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる


} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
  // ここで、s7はスコープを抜け、ドロップされる。s6もスコープを抜けるが、ムーブされているので、
  // 何も起きない。s5もスコープを抜け、ドロップされる。
fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする
    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string                              // some_stringが返され、呼び出し元関数に
    // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。
    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}
