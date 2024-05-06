fn main() {
    // パニックマクロを使用することが適する状況
    // プロトタイプコードやテストコード
    // コードが値に対して処理を行う際に、初めに値が合法化を確認し不正であればパニックする
    
    // パニックマクロを使用しないことが適する状況
    // コンパイラよりもプログラマがより情報を持っている場合
    // 不正な値を受け取った場合やHTTPリクエストのパースに失敗した場合など
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    // ゲッターメソッド
    pub fn value(&self) -> u32 {
        self.value
    }
}
