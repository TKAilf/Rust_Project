pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
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
// assert_eq!マクロは、2つの引数が等しいかどうかをチェックし、等しくない場合はパニックを起こす。
// つまり、値がtrueの場合は何も起こらず、falseの場合はpanicが発生する。
// bool値を返す関数をテストする場合は、assert!マクロを使うことが多いということである。

