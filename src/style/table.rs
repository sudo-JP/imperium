use std::path::PathBuf;

use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Alignment, Modify, Style};
use tabled::builder::Builder;
use owo_colors::OwoColorize;

use crate::commands::DEntry;
use crate::style::RGB;
use git2::Status;


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

fn truncate_path(path: &PathBuf) -> String {
    let home = std::env::var("HOME").unwrap_or_default();

    let path_str = path.to_string_lossy();
    let replaced = if path_str.starts_with(&home) {
        path_str.replacen(&home, "~", 1)
    } else {
        path_str.into_owned()
    };

    let term_width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80);

    let threshold = term_width / 3; 

    if replaced.len() > threshold {
        let components: Vec<&str> = replaced.split('/').collect();
        if components.len() > 3 {
            format!("{}/.../{}", components[0], components.last().unwrap())
        } else {
            replaced
        }
    } else {
        replaced
    }
}

fn color_name(dentry: &DEntry, record: &mut Vec<String>) -> String {
    if let Some(ref path) = dentry.symlink {
        record.push(truncate_path(path));
    format!("{}", dentry.name.truecolor(156, 207, 216))
    } else if dentry.is_dir {
        format!("{}", dentry.name.blue())
    } else if dentry.is_exec {
        format!("{}", dentry.name.truecolor(193, 225, 193))
    } else {
        dentry.name.clone()
    }
}

fn color_git(dentry: &DEntry) -> String {
    format!("{}", 
        if let Some(status) = dentry.git_status {
            match status {
                Status::WT_MODIFIED => "Modified".truecolor(246, 193, 119),
                Status::WT_NEW => "Untracked".truecolor(196, 167, 231), 
                Status::INDEX_NEW => "Staged".truecolor(49, 116, 143),
                Status::INDEX_MODIFIED => "Staged Modified".truecolor(49, 116, 143),

                _ => "Ignored".truecolor(110, 106, 134),
            }
        } else {
            "-".truecolor(110, 106, 134)
        }
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
            "Name".color(RGB::header_color()).to_string(),
            "Size".color(RGB::header_color()).to_string(),
            "Mode".color(RGB::header_color()).to_string(),
        ];

        if dentries.iter().any(|e| e.symlink.is_some()) {
            headers.push("Link".color(RGB::header_color()).to_string());
        }

        let have_git = if dentries.iter().any(|e| e.git_status.is_some()) {
            headers.push("Git".color(RGB::header_color()).to_string());
            true 
        } else { false };
        self.builder.push_record(headers);


        dentries.iter()
            .for_each(|dentry| {
                let size_str = dentry.size.to_string();
                let user_str = perm_to_string(dentry.user_perm >> 6);

                let mut record = vec![size_str, user_str];
                let rgb = RGB::hex_to_rgb(dentry.icon.color).unwrap();
                let name = color_name(dentry, &mut record);

                let info_name = format!("{}  {}", 
                    dentry.icon.truecolor(rgb.r(), rgb.g(), rgb.b()), 
                    name,
                );
                record.insert(0, info_name);

                // Git
                if have_git {
                    record.push(color_git(dentry));
                }

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


