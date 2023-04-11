// ファイル入出力:
// テキストファイルから行を読み込み、各行の文字数をカウントし、結果を新しいファイルに書き出すプログラム
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// output.txtにファイル内の各行の文字数を示
fn main() {
    let input_file = "./src/input.txt";
    let output_file = "./src/output.txt";

    match count_chars(input_file, output_file) {
        Ok(()) => println!("Character count written to {}", output_file),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn count_chars(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut output = File::create(output_file)?;

    for line in reader.lines() {
        let line = line?;
        writeln!(output, "{}", line.chars().count())?;
    }

    Ok(())
}
