// モジュールとクレート:
// 文字列を整数に変換する関数を作成し、エラー処理を行ってください。入力が不正な場合は適切なエラーメッセージを返す。
fn main() {
    let inputs = vec!["42", "hello", "123"];

    for input in inputs {
        match parse_integer(input) {
            Ok(number) => println!("Parsed number: {}", number),
            Err(err_msg) => println!("Error: {}", err_msg),
        }
    }
}

fn parse_integer(input: &str) -> Result<i32, String> {
    input.parse::<i32>().map_err(|_| format!("Invalid input: {}", input))
}

// Parsed number: 42
// Error: Invalid input: hello
// Parsed number: 123