mod args;
mod commands;
mod auxiliary;


use clap::Parser;
use commands::{init, hash_object};


use crate::args::Csync;
fn main() -> Result<(), std::io::Error> {
    let args = Csync::parse();
    match args {
        Csync::Init => init(),
        Csync::Hash(HashObject) => hash_object(HashObject),

    }
}
