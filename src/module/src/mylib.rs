// モジュールとクレート:
// 関数や構造体をモジュール化し、それを使用する外部のクレート
// mylibクレートを利用するために追加
extern crate mylib;

// mylibのutilsモジュールをインポート
use mylib::utils;

fn main() {
    // utilsモジュールの関数を使用
    let result = utils::factorial(5);
    println!("5! = {}", result);
}
