use chrono::{DateTime, Utc};

pub struct Commit {
    pub message: String,
    pub parent: Option<String>,
    pub author: String,
    pub tree: String,
    pub timestamp: DateTime<Utc>
}
