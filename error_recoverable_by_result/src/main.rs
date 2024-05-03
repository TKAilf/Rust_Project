use std::io;
use std::io::Read;
use std::fs::File;
// use std::io::ErrorKind;

fn main() {
    // let f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem: {:?}", e);
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // };

    // Shortcut for panic on error(unwrap & expect)
    // unwrap: Result値がOk列挙子の場合はOkの中身を返し、Err列挙子の場合はpanic!マクロを呼ぶ。
    // let _f = File::open("hello.txt").unwrap();

    // // expect: unwrapと同じだが、panic!マクロのメッセージを指定できる。
    // let _f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Delegate Errors(エラー委譲)
    let s = read_username_from_file();
    println!("{:?}", s);
    match s {
        Ok(s) => println!("Username: {}", s),
        Err(e) => println!("Error: {}", e),
    }

    // Shortcuts for Delegate Errors(エラー委譲のショートカット)]
    let s = read_username_from_file_shortcut();
    println!("{:?}", s);
    match s {
        Ok(s) => println!("Username: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Shortcuts for Delegate Errors(エラー委譲のショートカット)
// ?演算子を使用することで、Result値がOkの場合はOkの中身を返し、Errの場合はErrの中身を返す。
// ?演算子を使用したエラー値は、Fromトレイトを実装している場合に限り、自動的にFromトレイトを使用して変換される。
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    // 上記のコードは以下のように書き換えることもできる。
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
