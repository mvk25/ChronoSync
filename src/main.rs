mod args;
mod commands;
mod auxiliary;
mod blob;
mod index;

use std::fs;
use std::io::{Cursor, Write};

use blob::Blob;
use clap::Parser;
use commands::{init, add};
use index::{WarpIndex, INDEX_DATA, NO_TREE};
use crate::args::Commands::{Init, Hash, Add, UpdateIndex, WriteTree};
use crate::args::Warp;



#[warn(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut reader = Cursor::new(INDEX_DATA);
    // println!("{:#?}", WarpIndex::try_from(&mut reader));
    
    // let mut reader = Cursor::new(NO_TREE);
    // println!("{:#?}", WarpIndex::try_from(&mut reader));
    // // Ok(())
    let args = Warp::parse();
    match args.command {
        Init => init(),
        Hash { path } => {
                        // let hash_result = hash_object(path)?;
                        let new_blob = Blob::new(path);
                        Blob::compress_to_object(&new_blob);
                        Ok(())
            }
        Add { path } => add(path),
        UpdateIndex { add } => {
            WarpIndex::update_index(add);
            

            Ok(())
        },
        WriteTree => {
            // Creating an extension from an entry.c
            WarpIndex::write_tree();
            Ok(())
        }
    }
}
