use std::{env, fs, path::{Path, PathBuf}};

use crate::commands::hash_object;

// Create a file from the path endpoint
pub fn push_path(mut path: PathBuf, end_path: &str) {
    path.push(end_path);
    fs::File::create(path).unwrap();
}

// Create a single directory with a single file inside it(in Objects)
pub fn push_dir_with_file(mut path: PathBuf, dir: &str, file_name: &str) {
    path.push(dir);
    if !path.is_dir() {
        fs::create_dir(&path).expect("Error creating this directory");
    }
    push_path(path, file_name);
}

// Create directories with subsequent subdirectories
pub fn push_recursive_dir(mut path: PathBuf, dir: &str, end_points: Vec<&str>) {
    path.push(dir);
    match end_points.is_empty() {
        false => {
            fs::create_dir(&path).unwrap();
            end_points.iter().for_each(|endpoint| {
                let mut path_new = path.clone();
                path_new.push(endpoint);
                fs::create_dir(path_new).unwrap();
            });
        },
        true => {
            fs::create_dir(path).unwrap();
        }
    }
}

pub fn traverse_directory(path: &Path) {
    if path.is_dir() && path.file_name().unwrap() != ".warp" {
        let mut tree = path.file_name().unwrap();
        let mut tree_content: Vec<PathBuf> = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();

                    println!("Entry {:?} in dir {:?}", entry_path.file_name().unwrap(), path.file_name().unwrap());
                    if entry_path.is_file() {
                        let hash = hash_object(entry_path.clone()).expect("Error calculating hash of a file");
                        get_object_file_hash(hash);
                        tree_content.push(entry_path.clone());
                    } else if entry_path.is_dir() {
                        traverse_directory(&entry_path);
                        tree_content.push(entry_path);
                    }
                }
            }

            println!("{:?} {:?}", tree, tree_content);
        }
    } else {
        println!("{} is not a directory", path.display());
    }
}

fn get_object_file_hash(file_hash: String) {
    // Create an object directory and file for this file_hash
    let mut root = env::current_dir().unwrap();
    root.push(".warp");
    root.push("objects");

    let (dir_name, content) = file_hash.split_at(2);
    push_dir_with_file(root, dir_name, content);

}

pub fn file_exists(path: &PathBuf, file_name: &str) -> bool {
    let mut file_set = false;

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_name() == file_name {
                    file_set = true;
                    break;
                }
            }
        }
    }

    file_set
}