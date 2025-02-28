use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name= "warp")]
#[command(about = "Chrono Version Control")]
pub enum Csync {
    Init,
    Hash(HashObject)

}

#[derive(Parser, Debug)]
struct Init;

#[derive(Parser, Debug)]
pub struct HashObject {
    pub file: PathBuf
}
