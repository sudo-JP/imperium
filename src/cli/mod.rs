use clap::{Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "imperium ls")]
pub struct Cli {
    pub path: Option<PathBuf>,
}
