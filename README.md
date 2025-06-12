# 🦀 rsvcs — A Minimal Version Control System in Rust

[![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)

> A simplified Git-like version control system written from scratch in Rust.

---

## 📦 Features

| Command               | Description                               |
|-----------------------|-------------------------------------------|
| `rsvcs init`          | Initialize a new repository (`.mygit/`)   |
| `rsvcs add <file>`    | Stage a file for commit                   |
| `rsvcs commit -m ""`  | Commit staged files with a message        |
| `rsvcs log`           | View commit history                       |

---

## 🚀 Quick Start

### 📥 Clone & Build

```bash
git clone https://github.com/yourusername/rsvcs.git
cd rsvcs
cargo build --release
```

### 🧪 Example Usage
```bash
./target/release/rsvcs init
./target/release/rsvcs add hello.txt
./target/release/rsvcs commit -m "Initial commit"
./target/release/rsvcs log
```

***

## 📁 File Structure
```
.mygit/
├── commits/         # All committed snapshots (blobs)
├── index            # Staging area
└── HEAD             # Points to latest commit hash
```

## 🛠Tech Stack
- Rust
- clap - CLI Argument parsing
- serde - Serialisation for commits
- shai - Content Hashing
- chrono - Timestamps

## 📚 Roadmap
- [ ] `status` - staged vs modified files
- [ ] `checkout <hash>` - restore old state
- [ ] `diff` - show changes
- [ ] `branch` and `merge`
- [ ] `.rsvcsignore` support

## ✨ Contributing

Pull requests are welcome! If you'd like to:
- Add new cmds.
- Improve design 
- Refactor code

...feel free to fork and submit a PR.
