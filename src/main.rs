use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn echo(statement: String) {
    println!("{}", statement);
}

fn print_files(file_names: &[String]) {
    for file in file_names {
        let path = Path::new(file);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[2].clone();
    match command.as_str() {
        "echo" => echo(args[3].clone()),
        "cat" => print_files(&args[3..]),
        _ => println!("Not supported"),
    }
}
