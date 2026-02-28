use std::{collections::HashMap, fs::DirEntry, os::unix::fs::PermissionsExt, path::{Path, PathBuf}};
use anyhow::{Result};
use std::fs;
use devicons::FileIcon;
use git2::{Repository, Status, StatusOptions};

pub struct DEntry {
    pub user_perm: u32,
    pub size: u64,
    pub name: String,
    pub symlink: Option<PathBuf>,

    pub git_status: Option<git2::Status>,

    pub is_dir: bool,
    pub is_exec: bool,

    pub icon: FileIcon,
}

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
    fn from_path(path: &Path, name: &str, gitmap: &HashMap<String, Status>)-> Result<Self> {
        let metadata = fs::metadata(path)?;
        let perms = metadata.permissions().mode();

        let is_dir = metadata.is_dir();

        // Add more if needed 
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let icon = extension_icon(name, &ext, is_dir);

        let symlink_meta = fs::symlink_metadata(path)?;
        Ok(Self {
            user_perm: perms & USER_PERM,
            size: metadata.len(),
            name: name.to_string(),

            symlink: if symlink_meta.file_type().is_symlink() {
                fs::read_link(path).ok()
            } else { None },

            git_status: gitmap.get(name).copied(),
            is_dir,
            is_exec: (perms & EXECUTABLE) != 0,

            icon,
        })
    }

    fn new(entry: &DirEntry, gitmap: &HashMap<String, Status>) -> Result<Self> {
        let name = entry.file_name().into_string().unwrap_or("unknown name".into());
        Self::from_path(&entry.path(), &name, gitmap)
    }

    pub fn read_path(path: &PathBuf) -> Result<Vec<Self>> {
        let path = fs::canonicalize(path)?;

        // Check for git
        let map: HashMap<String, git2::Status> = Repository::discover(&path)
            .ok()
            .and_then(|r| {
                let mut opts = StatusOptions::new();
                opts.include_untracked(true);
                let statuses = r.statuses(Some(&mut opts)).ok()?;
                Some(statuses.iter()
                    .filter_map(|e| {
                        let path = e.path()?.to_string();
                        let name = Path::new(&path).file_name()?.to_string_lossy().into_owned();
                        Some((name, e.status()))
                    })
                    .collect())
            })
            .unwrap_or_default();


        // The ls itself
        let mut entries: Vec<Self> = fs::read_dir(&path)?
            .map(|entry| Self::new(&entry?, &map))
            .collect::<Result<Vec<_>>>()?;


        entries.sort_by_key(|e| !e.is_dir);
        entries.insert(0, Self::from_path(path.parent().unwrap_or(&path), "..", &map)?);
        entries.insert(0, Self::from_path(&path, ".", &map)?);

        Ok(entries)
    }
}
