// 変数、データ型、関数の基本:
// 与えられた数値の階乗を計算する関数
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("{}", factorial(4));
}
