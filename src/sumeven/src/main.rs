// 制御構造とループ:
// 引数として与えられた整数のリストから、偶数と奇数を分けてそれぞれの合計を計算する関数
fn sum_even_odd(numbers: &[i32]) -> (i32, i32) {
    let mut even_sum = 0;
    let mut odd_sum = 0;

    for &number in numbers {
        if number % 2 == 0 {
            even_sum += number;
        } else {
            odd_sum += number;
        }
    }

    (even_sum, odd_sum)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let (even_sum, odd_sum) = sum_even_odd(&numbers);

    println!("Even numbers sum: {}", even_sum);
    println!("Odd numbers sum: {}", odd_sum);
}
