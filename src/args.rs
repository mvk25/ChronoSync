use clap::Parser;

#[derive(Parser, Debug)]
#[command(name= "warp")]
#[command(about = "Chrono Version Control")]
pub enum Csync {
    Init,
}

#[derive(Parser, Debug)]
struct Init;