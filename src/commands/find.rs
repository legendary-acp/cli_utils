use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn execute(command: &[String]) {
    let dir = Path::new(&command[0]);
    let file_name = command[2].as_str();

    find(dir, file_name);
}

fn find(dir: &Path, name: &str) {
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();

                            if path.is_dir() {
                                find(&path, name);
                            } else if let Some(file_name) = path.file_name() {
                                if file_name == name {
                                    println!(
                                        "File found at {}",
                                        path.display().to_string().green()
                                    );
                                }
                            }
                        }
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory {}: {}", dir.display(), e),
        }
    }
}
