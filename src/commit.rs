use chrono::{Local, DateTime, Utc};
use sha1::{Sha1, Digest};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, path::Path};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub message: String,
    pub parent: Option<String>,
    pub author: String,
    pub tree: String,
    pub timestamp: DateTime<Utc>
}

pub fn commit_changes(message: &str) {
    let ind_path = Path::new("./.rsvcs/index");

    if !ind_path.exists() {
        println!("No changes to commit!");
        return;
    }

    let ind_contents = read_to_string(ind_path)
        .expect("Failed to read file");

    let mut files = HashMap::new();
}
