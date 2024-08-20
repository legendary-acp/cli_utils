mod commands;

use std::env;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[2].clone();
    match command.as_str() {
        "echo" => commands::echo::execute(args[3].clone()),
        "cat" => commands::cat::execute(&args[3..]),
        "ls" => commands::ls::execute(&args[3..]),
        "grep" => commands::grep::execute(args[3].clone(), args[4].clone()),
        _ => println!("Not supported"),
    }
}
