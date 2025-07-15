mod args;
mod commands;
mod auxiliary;
mod blob;
mod index;
mod commit;

use std::fs;
use std::io::{Cursor, Read, Write};

use blob::Blob;
use clap::Parser;
use commands::{init, add};
use index::{WarpIndex, INDEX_DATA, NO_TREE};
use crate::args::Commands::{Init, Hash, Add, UpdateIndex, WriteTree, TestTree, CommitTree};
use crate::args::Warp;
use crate::commit::Commit;



#[warn(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        },
        CommitTree { tree, parents, message } => {
            let new_commit = Commit::new(tree, parents, message);
            Commit::compress_to_object(&new_commit);
            Ok(())
        },
        TestTree { path } => {
            let contents = fs::read(path).unwrap();
            let mut cursor = Cursor::new(contents.as_slice());
            println!("{:?}", WarpIndex::try_from(&mut cursor).unwrap());
            Ok(())
        }
    }
}
