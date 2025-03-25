use std::{fs, path::Path};

pub fn generate(path: &str) {
    println!("Creating a new shield at: {}", path);
    let file_path = Path::new(path);
    if file_path.exists() {
        println!("The shield already exists at: {}", path);
    } else {
        fs::create_dir_all(file_path).expect("Failed to create directory");
        println!("Shield created at: {}", path);
    }
}
