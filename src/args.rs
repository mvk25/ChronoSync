use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};


#[derive(Debug, Parser)]
#[command(name = "warp")]
#[command(about = "Chrono Version Control")]
pub struct Warp {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init,
    Hash {
        #[arg(required = true)]
        path: PathBuf
    },
    Add {
        #[arg(required = true)]
        path: Vec<PathBuf>
    },
    UpdateIndex {
        #[arg(
            long,
            value_name = "FILENAME",
            num_args = 1..,
            help = "Stage a file"
        )]
        add: Vec<PathBuf>

    }
}
