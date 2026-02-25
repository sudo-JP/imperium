use clap::Parser;
use owo_colors::OwoColorize;
use imperium::{cli::Cli, commands::ls};

fn main() {
    let cli = Cli::parse();
    let v = ls::DEntry::read_path(&cli.path.unwrap_or(".".into())).unwrap();
    v.iter().for_each(|e| println!("{} {}", e.name, e.is_exec));
}
