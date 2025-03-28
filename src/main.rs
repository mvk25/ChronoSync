mod args;
mod commands;
mod auxiliary;
mod blob;


use blob::Blob;
use clap::Parser;
use commands::{init, add};
use crate::args::Commands::{Init, Hash, Add, UpdateIndex};
use crate::args::Warp;


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
        UpdateIndex {  } => todo!(),
    }
}
