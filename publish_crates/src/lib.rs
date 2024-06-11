// cargo doc --openで現在のクレートのドキュメント用のHTMLを構築し、webブラウザで開くことができる
// Panic, Errors, Safetyなどクレートを使用する上での注意点を記載することが多い
// //!はドキュメンテーションコメントであり、クレートやモジュール自体を解説するのに有用なものである

//! # publish_crates
//!
//! `publish_crates` is a collection of utilities to make performing certain
//! calculations more convenient.

//! # 自分のクレート
//!
//! `publish_crates`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。
//! # Art
//!
//! A library for modeling artistic concepts.
//! # 芸術
//!
//! 芸術的な概念をモデル化するライブラリ。

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

/// Adds one to the number given.
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, publish_crates::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    /// RYBカラーモデルによる主色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    /// RYBカラーモデルによる副色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    /// 2つの主色を同じ割合で混合し、副色にする
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) | (PrimaryColor::Yellow, PrimaryColor::Red) => {
                SecondaryColor::Orange
            }
            (PrimaryColor::Red, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Red) => {
                SecondaryColor::Purple
            }
            (PrimaryColor::Yellow, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Yellow) => {
                SecondaryColor::Green
            }
            // 同じ色の組み合わせの場合は適当な値を返す
            _ => panic!("Invalid color combination"),
        }
    }
}
