use clap::Parser;
use imperium::{cli::Cli, commands::ls, style::DisplayTable};

fn main() {
    let cli = Cli::parse();
    let dentries = ls::DEntry::read_path(&cli.path.unwrap_or(".".into()))
        .unwrap();
    let mut display = DisplayTable::new();
    display.render_ls(&dentries);
    display.display();
}
