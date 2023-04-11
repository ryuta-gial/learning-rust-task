use std::sync::{Arc, Mutex};
use std::thread;
// 合計値を計算
// The sum of the numbers from 1 to 100 is: 5050
fn main() {
    // ベクターに1から100までの連続した整数を格納
    let numbers: Vec<i32> = (1..=100).collect();
    let thread_count = 4;

    let total_sum = multi_threaded_sum(&numbers, thread_count);
    println!("The sum of the numbers from 1 to 100 is: {}", total_sum);
}

fn multi_threaded_sum(numbers: &[i32], thread_count: usize) -> i32 {
    let numbers = Arc::new(numbers.to_vec());
    let sum = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    let chunk_size = (numbers.len() + thread_count - 1) / thread_count;

    for i in 0..thread_count {
        let numbers_clone = Arc::clone(&numbers);
        let sum_clone = Arc::clone(&sum);

        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = std::cmp::min(start + chunk_size, numbers_clone.len());
            let partial_sum: i32 = numbers_clone[start..end].iter().sum();

            let mut sum = sum_clone.lock().unwrap();
            *sum += partial_sum;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // `MutexGuard`のライフタイムを制限するために、`sum`の値をローカル変数に格納してから返す。
    let total_sum = *sum.lock().unwrap();
    total_sum
}
