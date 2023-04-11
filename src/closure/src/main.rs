// クロージャ:
// クロージャを使用して、与えられた整数のリスト内の各要素を指定された値で乗算する関数
// 元の数値が[1, 2, 3, 4, 5]で、適用後の数値が[3, 6, 9, 12, 15]になります。
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let multiplier = 3;

    let result = apply_multiplier(&numbers, multiplier);
    println!("Original numbers: {:?}", numbers);
    println!("After applying multiplier {}: {:?}", multiplier, result);
}

fn apply_multiplier(numbers: &[i32], multiplier: i32) -> Vec<i32> {
    let multiply = |x: i32| x * multiplier;
    numbers.iter().map(|&x| multiply(x)).collect()
}
