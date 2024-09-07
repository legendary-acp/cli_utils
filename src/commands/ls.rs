use colored::Colorize;
use std::fs;
use std::path::Path;

//TODO: Extend the functionality in future to get all these values using metadata
// struct Entity {
//     permissions: String,
//     created_at: String,
//     accessed_at: String,
//     size: u64,
//     is_folder: bool,
//     name: String
// }

pub fn execute(dirs: &[String]) {
    for dir in dirs {
        if dir == "." {
            println!("\nPresent working directory contains following content:");
        } else {
            println!("\n{} contains following content:", dir);
        }
        list_dir(dir);
    }
    if dirs.len() == 0 {
        println!("Present working directory contains following content:");
        let dir = &String::from(".");
        list_dir(dir);
    }
}

fn list_dir(dir: &String) {
    let path = Path::new(dir);

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            println!("{}", path.display().to_string().green());
                        } else {
                            println!("{}", path.display().to_string().blue());
                        }
                    }
                    Err(e) => println!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
}
