extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

// コマンドラインで以下のコマンドを実行することで環境変数を変更できる
// $env:CASE_INSENSITIVE=1
fn main() {
    let args: Vec<String> = env::args().collect();

    // let (query, filename) = parse_config(&args);
    // let config = Config::new(&args);
    // Result型を返すようにリファクタリングしたことによる対応
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

    // run(config).unwrap_or_else(|err| {
    //     println!("Application error: {}", err);
    //     process::exit(1);
    // })
}

