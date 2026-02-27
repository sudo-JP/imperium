use std::{fs::DirEntry, os::unix::fs::{MetadataExt, PermissionsExt}, path::{Path, PathBuf}};
use chrono::prelude::{DateTime, Utc};
use anyhow::{Result};
use std::fs;
use devicons::FileIcon;

pub struct DEntry {
    pub user_perm: u32,
    pub group_perm: u32, 
    pub other_perm: u32,
    pub hardlinks: u64,
    pub size: u64,
    pub mdate: DateTime<Utc>, 
    pub name: String,

    pub is_symlink: bool,
    pub is_dir: bool,
    pub is_exec: bool,

    pub icon: FileIcon,
}

const OTHER_PERM: u32 = 0o007;
const GROUP_PERM: u32 = 0o070;
const USER_PERM: u32 = 0o700;
const EXECUTABLE: u32 = 0o100;

fn extension_icon(name: &str, ext: &str, is_dir: bool) -> FileIcon {
    if is_dir { return FileIcon::from(Path::new(".")); } 
    match name {
        "Makefile" | "makefile" => { return FileIcon{icon: '󱌣', color: "#393552"}; }
        _ => {}
    }
    match ext {
        "js" => FileIcon{icon: '', color: "#f7df1e"},
        _ => FileIcon::from(name.to_ascii_lowercase()),
    }
}

impl DEntry {
    fn from_path(path: &Path, name: &str) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let perms = metadata.permissions().mode();
        let mdate: DateTime<Utc> = metadata.modified()?.into();

        let is_dir = metadata.is_dir();

        // Add more if needed 
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let icon = extension_icon(name, &ext, is_dir);

        Ok(Self {
            user_perm: perms & USER_PERM,
            group_perm: perms & GROUP_PERM,
            other_perm: perms & OTHER_PERM,
            hardlinks: metadata.nlink(),
            size: metadata.len(),
            mdate,
            name: name.to_string(),

            is_symlink: metadata.is_symlink(),
            is_dir,
            is_exec: (perms & EXECUTABLE) != 0,

            icon,
        })
    }

    fn new(entry: &DirEntry) -> Result<Self> {
        let name = entry.file_name().into_string().unwrap_or("unknown name".into());
        Self::from_path(&entry.path(), &name)
    }

    pub fn read_path(path: &PathBuf) -> Result<Vec<Self>> {
        let path = fs::canonicalize(path)?;

        let mut entries: Vec<Self> = fs::read_dir(&path)?
            .map(|entry| Self::new(&entry?))
            .collect::<Result<Vec<_>>>()?;


        entries.sort_by_key(|e| !e.is_dir);
        entries.insert(0, Self::from_path(path.parent().unwrap_or(&path), "..")?);
        entries.insert(0, Self::from_path(&path, ".")?);

        Ok(entries)
    }
}
