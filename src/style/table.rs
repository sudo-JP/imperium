use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Alignment, Modify, Style};
use tabled::builder::Builder;
use anyhow::Result;
use owo_colors::OwoColorize;

use crate::commands::DEntry;

struct RGB {
    r: u8,
    g: u8, 
    b: u8,
}

impl RGB {
    pub fn hex_to_rgb(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches("#");

        let num = u32::from_str_radix(hex, 16)?;

        let r = ((num >> 16) & 0xFF) as u8;
        let g = ((num >> 8) & 0xFF) as u8;
        let b = (num & 0xFF) as u8;

        Ok(Self { r, g, b })
    }

    pub fn r(&self) -> u8 { self.r }
    pub fn g(&self) -> u8 { self.g }
    pub fn b(&self) -> u8 { self.b }
}


fn perm_to_string(perm: u32) -> String {
    format!(
        "{}{}{}",
        if perm & 0o4 != 0 { "r".truecolor(235, 111, 146) } 
        else { "-".truecolor(110, 106, 134) },
        if perm & 0o2 != 0 { "w".truecolor(246, 193, 119) } 
        else { "-".truecolor(110, 106, 134) },
        if perm & 0o1 != 0 { "x".truecolor(193, 225, 193) } 
        else { "-".truecolor(110, 106, 134) },
    )
}

pub struct DisplayTable {
    builder: Builder,
}

impl DisplayTable {
    pub fn new() -> Self {
        Self { builder: Builder::new(), }
    }

    pub fn render_ls(&mut self, dentries: &[DEntry]) {
        let mut headers = vec![
            "Name".color(owo_colors::Rgb(196, 167, 231)).to_string(),
            "Size".color(owo_colors::Rgb(196, 167, 231)).to_string(),
            "Mode".color(owo_colors::Rgb(196, 167, 231)).to_string(),
        ];
        if dentries.iter().any(|e| e.symlink.is_some()) {
            headers.push("Link".color(owo_colors::Rgb(196, 167, 231)).to_string());
        }
        self.builder.push_record(headers);


        dentries.iter()
            .for_each(|dentry| {
                let size_str = dentry.size.to_string();
                let user_str = perm_to_string(dentry.user_perm >> 6);

                let mut record = vec![size_str, user_str];
                let rgb = RGB::hex_to_rgb(dentry.icon.color).unwrap();
                let name = if let Some(ref p) = dentry.symlink {
                    record.push(p.to_string_lossy().into_owned());
                    format!("{}", dentry.name.truecolor(156, 207, 216))
                } else if dentry.is_dir {
                    format!("{}", dentry.name.blue())
                } else if dentry.is_exec {
                    format!("{}", dentry.name.truecolor(193, 225, 193))
                } else {
                    dentry.name.clone()
                };

                let info_name = format!("{}  {}", 
                    dentry.icon.truecolor(rgb.r(), rgb.g(), rgb.b()), 
                    name,
                );
                record.insert(0, info_name);

                self.builder.push_record(record);
            });
    }

    pub fn display(self) {
        let table = self.builder.build()
            .with(Style::rounded())
            .with(Modify::new(Columns::one(1)).with(tabled::settings::Color::rgb_fg(235, 188, 186)))
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::one(2)).with(Alignment::center()))
            .to_string();
        println!("{}", table);
    }
}


