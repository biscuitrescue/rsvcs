use std::{path::Path, fs::{create_dir_all, write}};

pub fn init_repo() {
    let root = Path::new(".rsvcs");
    if root.exists() {
        println!("Repo already initialised!");
        return;
    }

    create_dir_all(root.join("commits"))
        .expect("Failed to create {root}/commits/");
    write(root.join("index"), b"")
        .expect("Failed to create {root}/index");
    write(root.join("HEAD"), b"")
        .expect("Failed to create {root}/HEAD");

    println!("Repo Initialised successfully");
}
