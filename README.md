# ChronoSync ⏳🔗

ChronoSync is a lightweight version control system inspired by Git, designed to provide essential version control functionality with a clean and intuitive interface. Instead of using `.git` directories, ChronoSync uses `.warp` directories to store version control information.

## ✨ Features

ChronoSync currently implements these core features:

- **📂 Repository Initialization**  
  Create a new repository with `warp init`
- **📝 Index Management**  
  Track files and their states in a sophisticated index structure
- **🗄️ Object Storage**  
  Store file content as compressed objects using zlib compression
- **🔄 Working Directory Integration**  
  Create an index from the current state of the working directory

## 🛠️ How ChronoSync Works

ChronoSync follows a similar model to Git but with its own implementation:

### 📁 Repository Structure

When you initialize a ChronoSync repository with `warp init`, it creates a `.warp` directory with the following structure:



### Core Components

1. **Index File**
   - Binary format storing file metadata
   - Tracks:
     - File paths
     - Timestamps
     - SHA-1 hashes
     - File permissions

2. **Object Storage**
   - Files are compressed using zlib
   - Stored in `.warp/objects/`
   - Content-addressable (named by hash)


---

## 🔧 Development Status

| Feature           | Status       |
|-------------------|--------------|
| Repository Init   | ✅ Done      |
| Index Management  | ✅ Done      |
| Object Storage    | ✅ Done      |
| Commit System     | 🚧 In Progress |
| Branching         | ❌ Planned   |
| Merge Operations  | ❌ Planned   |

## Getting Started

### Installation

```bash
git clone https://github.com/mvk25/ChronoSync
cd ChronoSync
cargo run 

# 🛠️ Running ChronoSync

Once you’ve built the project with Cargo, you can run ChronoSync directly from the terminal.

```bash
$ ~/<file_location>/target/release/ChronoSync --help
```

### 📖 Help Output

```
Chrono Version Control

Usage: ChronoSync <COMMAND>

Commands:
  init            Initialize a new ChronoSync repository
  hash            Compute object hash of a file
  add             (Planned) Add files to staging
  update-index    Update the index file from working directory
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help      Print help
```

---

## 📦 Example Usage

### Initialize a repository

```bash
$ ~/<file_location>/target/release/ChronoSync init
```

### Compute a hash of a file

```bash
$ ~/<file_location>/target/release/ChronoSync <FILENAME>
```

### Update index with a file

```bash
$ ~/<file_location>/target/release/ChronoSync update-index --add <FILENAMES>
```

### Write to a tree after update
```bash
$ ~/<file_location>/target/release/ChronoSync write-tree
```
