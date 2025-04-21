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
   - Stored in `.warp/objects/blobs/`
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
git clone https://github.com/your-repo/chronosync
cd chronosync
cargo install --path .