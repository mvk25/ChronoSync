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
            required = true,
            num_args = 1..,
            help = "Stage a file"
        )]
        add: Vec<PathBuf>
    },
    WriteTree,
    CommitTree {
        tree: String,
        #[arg(
            short = 'p',
            long,
            value_name = "parent",
            required = false,
            help = "ID of parent commit object"
        )]
        parents: Option<String>,
        #[arg(
            short = 'm',
            long,
            value_name = "message",
            required = false,
            help = "Commit message"
        )]
        message: String
    },
    TestTree {
        #[arg(required = true)]
        path: PathBuf
    },
}
