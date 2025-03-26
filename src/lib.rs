mod args;
pub mod commands;
mod auxiliary;
mod blob;
mod tree;
use std::path::PathBuf;
use std::sync::OnceLock;

pub static ROOT: OnceLock<PathBuf> = OnceLock::new();
