use sha1::{Sha1, Digest};
use std::{fs::{create_dir_all, read, write}, io::Write, path::Path};

pub fn add_file(file: &str) {
    let path = Path::new(file);

    if !path.exists() {
        println!("File not found: {file}");
        return;
    }

    let contents = read(path)
        .expect("Failed to read contents");

    // Computes hash
    let mut hasher = Sha1::new();
    hasher.update(&contents);
    let hash = hex::encode(hasher.finalize());

    let obj_dir = Path::new(".rsvcs/objects");
    if !obj_dir.exists() {
        create_dir_all(obj_dir)
            .expect("Failed to create {obj_dir}");
    }
    
    let obj_path = obj_dir.join(&hash);
    if !obj_path.exists() {
        write(&obj_path, &contents)
            .expect("Failed to write blob");
    }

    let mut index = std::fs::OpenOptions::new() // TODO: use proper serialisation -> serde
        .append(true)
        .create(true)
        .open(".rsvcs/index")
        .expect("Failed to open index!");

    writeln!(index, "{} {}", hash, file)
        .expect("Failed to append to index");

    println!("Added file: {} (hash: {})", file, hash);
}
