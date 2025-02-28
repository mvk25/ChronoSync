mod args;
mod commands;
mod auxiliary;
use clap::Parser;
use commands::init;


use crate::args::Csync;
fn main() -> Result<(), std::io::Error> {
    let args = Csync::parse();
    match args {
        Csync::Init => init(),
    }
}
