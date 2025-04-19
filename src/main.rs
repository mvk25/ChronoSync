mod args;
mod commands;
mod auxiliary;
mod blob;
mod index;

use std::io::Cursor;
use std::panic::UnwindSafe;
use std::vec;

use blob::Blob;
use clap::Parser;
use commands::{init, add};
use index::{IndexEntry, IndexHeader};
use crate::args::Commands::{Init, Hash, Add, UpdateIndex};
use crate::args::Warp;



#[warn(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut reader = Cursor::new(crate::index::INDEX_DATA);
    let test_header = IndexHeader::try_from(&mut reader).unwrap();
    let mut vecEntries: Vec<IndexEntry> = Vec::new();
    for __ in 0..test_header.entry_count {
        vecEntries.push(IndexEntry::try_from(&mut reader).unwrap());
    }
    // let test_entry = IndexEntry::try_from(&mut reader).unwrap();

    println!("{:?}", test_header);

    for entry in vecEntries { 
        println!("{:?}", entry);
    }
    Ok(())
    // let args = Warp::parse();
    // match args.command {
    //     Init => init(),
    //     Hash { path } => {
    //                     // let hash_result = hash_object(path)?;
    //                     let new_blob = Blob::new(path);
    //                     Blob::compress_to_object(&new_blob);
    //                     Ok(())
    //         }
    //     Add { path } => add(path),
    //     UpdateIndex {  } => todo!(),
    // }
}
