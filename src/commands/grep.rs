use std::fs::read_to_string;

pub fn execute(file_name: String, text: String) {
    for line in read_to_string(file_name).unwrap().lines() {
        let cur_line = line.to_string();
        if cur_line.contains(text.as_str()) {
            println!("{}", cur_line)
        }
    }
}
