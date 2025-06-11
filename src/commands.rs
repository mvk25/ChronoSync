use std::ffi::CString;
use std::path::PathBuf;
use std::env;
use std::env::VarError;
use std::fs;
use std::io::{Error, Read};
use sha1::{Sha1, Digest};
use std::sync::OnceLock;
use colored::Colorize;

#[allow(dead_code)]

use crate::auxiliary::{push_recursive_dir, push_path, traverse_directory};

pub static ROOT: OnceLock<PathBuf> = OnceLock::new();

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // (Importante): Later on, we have to add a feature, where we first scan if .warp exists on an upper tree
    // wherever this function is called on, and restrict it from continuing, since we have a .warp file
    // already initialised. We know to have a way of knowing the root directory(This should be known by 
    // the first init call), OnceLock is not working!!.

    // Also we should propagate up the cwd to find the .warp file instead of what we are currently doing.

    // Initialise our working directory
    if ROOT.set(env::current_dir().unwrap()).is_err() {
        panic!("ROOT is already initialised");
    }
    // Create the .warp directory
    // If we dont have a WARP_DIRECTORY env var, we just use .warp
    // We are currently using io::Errors
    let mut warp_dir  = String::new();
    let warp_directory = match env::var("WARP_DIR") {
        Ok(key) => {
            warp_dir = key.clone();
            Ok(fs::create_dir(key).unwrap())
        },
        Err(e) => {
            match e {
                VarError::NotPresent => {
                    warp_dir += ".warp";
                    Ok(fs::create_dir(".warp").expect("Directory already exists"))
                },
                VarError::NotUnicode(err_msg) => Err(Error::new(std::io::ErrorKind::Other, err_msg.to_str().unwrap()))
            }
        }
    };

    match warp_directory {
        Ok(_) => {
            // create config, description and HEAD files
            let mut root = ROOT.get().expect("Init command not called").to_owned();
            root.push(  warp_dir.as_str());
            
            push_path(root.clone(), "config");
            push_path(root.clone(), "description");
            push_path(root.clone(), "HEAD");

            // Create subdirectories objects, refs/heads, refs/tags
            push_recursive_dir(root.clone(), "refs", vec!["heads", "tags"]);
            push_recursive_dir(root.clone(), "objects", vec!["info", "pack"]);
            push_recursive_dir(root.clone(), "branches", vec![]);

            // We should probably print something to the terminal here.
            println!("{}", "hint: Using .warp as the default directory for Warp data.".yellow());
            println!("{}", "hint: Feature for customizing default directory should be added soon".yellow());
            println!("Initialised empty Warp repository in {}", root.display().to_string().yellow());
            
        },
        Err(_) => todo!(),
    }
    


    Ok(())
}

// We are going to generate a hash for a blob with this function
pub fn hash_object(args: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(args).unwrap();

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    
    let blob = buf.as_bytes();
    let blob_len = blob.len();

    let header = format!("blob {}", blob_len);
    let header = CString::new(header).expect("CString failed");

    let header_bytes = header.as_bytes_with_nul();
    let hash_object = [header_bytes, blob].concat();
    
    hasher.update(hash_object);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

pub fn add(args: Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    // let root = ROOT.get().expect("Unable to get the current working directory");
    let root = env::current_dir().expect("Unable to get the current working directory");
    println!("{:?}", root.file_name().unwrap());
    // // create a index file inside the .git if it does not exists
    // let mut warp_dir = root.clone();
    // warp_dir.push(".warp");

    // if !file_exists(&warp_dir, "index") {
    //     push_path(warp_dir.clone(), "index");
    // }
    
    traverse_directory(&root);

    Ok(())
}