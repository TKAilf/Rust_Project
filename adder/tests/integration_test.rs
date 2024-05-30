extern crate adder;
mod common;

#[test]
fn it_adder_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// 結合テスト
// Rustにおいて結合テストは完全にライブラリ外のものです。
// ライブラリのいろいろな部分が共同で正常に動作しているかをテストします。
// そのためにまずはtestsディレクトリを作成し、そのなかにtest用のコードを記載します。
// cargo test --test ファイル名 : testsディレクトリ内に存在する「ファイル名」のファイルをテスト実行する
// 今回の場合はcargo test --test integration_test

// testsディレクトリ内の各ファイルは、個別のクレートとしてコンパイルされる。

