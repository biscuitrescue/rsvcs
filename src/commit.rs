// use chrono::{Local, DateTime, Utc};
use sha1::{Sha1, Digest};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, path::Path};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub message: String,
    pub files,
    // pub timestamp: DateTime<Utc>
}

pub fn commit_changes(message: &str) {
    let ind_path = Path::new("./.rsvcs/index");

    if !ind_path.exists() {
        println!("No changes to commit!");
        return;
    }

    let content = read_to_string(ind_path)
        .expect("Failed to read file");

    let mut files = HashMap::new();

    for line in content.lines() {
        let val: Vec<&str> = line.split_whitespaces().collect();
        if val.len() == 2 {
            files.insert(val[1].to_string(), val[0].to_string());
        }
    }

    // Add timestamp

    let commit = Commit {
        message: message.to_string(),
        files,
    };

    // hashing left
}

