use std::env;

fn echo(statement: String){
    println!("{}", statement);
}


fn main() {
    let args:Vec<String>=env::args().collect();
    let command = args[2].clone();
    match command.as_str() {
        "echo" => echo(args[3].clone()),
        _ => println!("Not supported"),
    }
}
