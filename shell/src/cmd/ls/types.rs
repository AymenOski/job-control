use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub a: bool,
    pub l: bool,
    pub f: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LongEntry {
    pub perms: String,
    pub links: String,
    pub user: String,
    pub group: String,
    pub size: String,
    pub date: String,
    pub name: String,
    pub blocks: u64,
    pub path: PathBuf,
}