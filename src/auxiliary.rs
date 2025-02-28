use std::{fs, path::{Path, PathBuf}};

pub fn push_path(mut path: PathBuf, end_path: &str) {
    path.push(end_path);
    fs::File::create(path).unwrap();
}

pub fn push_dir(mut path: PathBuf, dir: &str, end_points: Vec<&str>) {
    path.push(dir);
    match end_points.is_empty() {
        false => {
            fs::create_dir(&path).unwrap();
            end_points.iter().for_each(|endpoint| {
                let mut path_new = path.clone();
                path_new.push(endpoint);
                println!("{}", path_new.display());
                fs::create_dir(path_new).unwrap();
            });
        },
        true => {
            fs::create_dir(path).unwrap();
        }
    }
}