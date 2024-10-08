fn main() {
    println!("Hello, world!");
}

// パッケージとは、Cargoが管理するコードの単位であり、Cargo.tomlファイルで定義される
// パッケージはこのフォルダでいうと、my_projectという名前のフォルダに対応する
// パッケージは複数のクレートを持つことができる
// クレートには、ライブラリクレートとバイナリクレートがある
// パッケージは、ライブラリクレートとバイナリクレートのどちらかを持つ必要がある
// ライブラリクレートは複数持つことができるが、バイナリクレートは1つしか持つことができない

// src/main.rsが存在する場合、慣習的にmain.rsをバイナリクレートルートとして扱う
// バイナリクレートはこのフォルダでいうと、my_project/target/debug/my_project.exeである

// src/lib.rsが存在する場合、慣習的にlib.rsをライブラリクレートルートとして扱う

// src/binディレクトリが存在する場合、その中の各ファイルがバイナリクレートとして扱われる
// この場合、src/binディレクトリ中の各バイナリファイルがバイナリクレートルートとして扱われる
