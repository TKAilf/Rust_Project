extern crate adder;

#[test]
fn it_adder_two() {
    assert_eq!(4, adder::add_two(2));
}

// cargo test --test ファイル名 : testsディレクトリ内に存在する「ファイル名」のファイルをテスト実行する
// 今回の場合はcargo test --test integration_test
