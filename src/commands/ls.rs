pub fn execute(dirs: &[String]) {
    for dir in dirs {
        println!("{} contains following files:", dir);
        list_dir(dir);
    }
}

fn list_dir(_dir: &String) {}
