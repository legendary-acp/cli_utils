use colored::Colorize;
use std::fs::read_to_string;

fn print_highlight_word(sentence: &str, word: &str) {
    if let Some(pos) = sentence.find(word) {
        let before = &sentence[..pos];
        let after = &sentence[pos + word.len()..];

        let red_word = word.blue();

        println!("{}{}{}", before, red_word, after);
    } else {
        println!("{}", sentence);
    }
}

pub fn execute(file_name: String, text: String) {
    for line in read_to_string(file_name).unwrap().lines() {
        let cur_line = line.to_string();
        if cur_line.contains(text.as_str()) {
            print_highlight_word(cur_line.as_str(), text.as_str());
        }
    }
}
