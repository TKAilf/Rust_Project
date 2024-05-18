pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// 関数をテスト関数にするためには
// fnの前に#[test]をつける
// cargo testコマンドでテストを実行できる

// また、新しいライブラリプロジェクトをCargoで作成すると、上記のように
// テストコードが自動で生成されるため、新しくプロジェクトを始めるたびに
// テスト関数の正しい構造や文法をいちいち検索する必要がない。

// testsモジュール内において、#[test]がついていない関数はテストされない。
// これはtest関数をテストする際に、一般的なシナリオをセットアップしたり共通の処理を行う手助けをしたりできるということである。

// 関数本体は、assert_eq!マクロを使って期待する結果を検証している。
