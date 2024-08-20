use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn execute(file_names: &[String]) {
    for file in file_names {
        read_file(file)
    }
}

fn read_file(file: &String) {
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
