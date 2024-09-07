mod commands;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    println!("{}", command.clone());
    match command.as_str() {
        "echo" => commands::echo::execute(args[2].clone()),
        "cat" => commands::cat::execute(&args[2..]),
        "ls" => commands::ls::execute(&args[2..]),
        "grep" => commands::grep::execute(args[2].clone(), args[2].clone()),
        _ => println!("Not supported"),
    }
}
