use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    word_count
}

// 単語の出現回数をカウント
fn main() {
    let text = "This is a sample text. This text contains some words. Some words may be repeated.";

    let word_count_map = count_words(text);

    println!("Word count for the given text:");
    for (word, count) in &word_count_map {
        println!("{}: {}", word, count);
    }
}

// Word count for the given text:
// Some: 1
// repeated.: 1
// may: 1
// a: 1
// contains: 1
// is: 1
// some: 1
// sample: 1
// text: 1
// words.: 1
// words: 1
// This: 2
// text.: 1
// be: 1