use std::env;
use std::env::VarError;
use std::fs;
use std::io::Error;

use crate::auxiliary::{push_dir, push_path};

pub fn init() -> Result<(), std::io::Error> {
    // Initialise our working directory
    let mut path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    
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
                    Ok(fs::create_dir(".warp").unwrap())
                },
                VarError::NotUnicode(err_msg) => Err(Error::new(std::io::ErrorKind::Other, err_msg.to_str().unwrap()))
            }
        }
    };

    match warp_directory {
        Ok(_) => {
            // create config, description and HEAD files
            path.push(  warp_dir.as_str());
            
            push_path(path.clone(), "config");
            push_path(path.clone(), "description");
            push_path(path.clone(), "HEAD");

            // Create subdirectories objects, refs/heads, refs/tags
            push_dir(path.clone(), "refs", vec!["heads", "tags"]);
            push_dir(path.clone(), "objects", vec!["info", "pack"]);
            push_dir(path.clone(), "branches", vec![]);
            println!("{}", path.display());
            
        },
        Err(_) => todo!(),
    }
    


    Ok(())
}