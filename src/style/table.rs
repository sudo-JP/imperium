use tabled::settings::object::Cell;
use tabled::settings::span::ColumnSpan;
use tabled::settings::style::BorderColor;
use tabled::settings::{Alignment, Modify, Style};
use tabled::{builder::Builder, Table};
use tabled::settings::themes::Colorization;

use crate::commands::DEntry;

pub struct DisplayTable {
    builder: Builder,
}

impl DisplayTable {
    pub fn new() -> Self {
        Self { builder: Builder::new() }
    }

    pub fn render_ls(&mut self, dentries: &[DEntry]) {
        self.builder.push_record(["Name", "Size", "User", "Group", "Other", "M.Time", "Owner", "Hardlinks"]);
        dentries.iter()
            .for_each(|dentry| {
                let name = format!("{} {}", dentry.icon, dentry.name);
                self.builder.push_record([
                    &name,
                    
                    &dentry.size.to_string()
                ]);
            });
    }

    pub fn display(self) {
        let table = self.builder.build()
            .with(Style::rounded())
            /*.with(BorderColor::default().top(self.border_color.clone())
                .left(self.border_color.clone())
                .right(self.border_color.clone())
                .bottom(self.border_color.clone())
                .corner_top_left(self.border_color.clone())
                .corner_top_right(self.border_color.clone())
                .corner_bottom_left(self.border_color.clone())
                .corner_bottom_right(self.border_color.clone()))*/
            .with(Modify::new(Cell::new(0, 2)).with(Alignment::center()))
            .to_string();
        println!("{}", table);
    }
}


