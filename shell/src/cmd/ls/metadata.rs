use crate::error::ShellError;

use chrono::{DateTime, Duration, Local};

use std::{
    ffi::CStr,
    fs,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    time::SystemTime,
};

use super::{
    formatter::append_indicator,
    types::{Flags, LongEntry},
};

pub fn build_long_entry(path: &Path, flags: &Flags) -> Result<LongEntry, ShellError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|e| ShellError::Other(format!("ls: cannot access '{}': {}", path.display(), e)))?;
    let mut name = file_name_or_path(path);

    if flags.f {
        
        append_indicator(&mut name, &metadata);
    }

    if metadata.file_type().is_symlink() {
        if let Ok(target) = fs::read_link(path) {
            name.push_str(" -> ");
            name.push_str(&target.to_string_lossy());
        }
    }

    Ok(LongEntry {
        perms: format_permissions(&metadata),
        links: metadata.nlink().to_string(),
        user: owner_name(&metadata).unwrap_or_else(|_| metadata.uid().to_string()),
        group: group_name(&metadata).unwrap_or_else(|_| metadata.gid().to_string()),
        size: format_size(&metadata),
        date: format_date(metadata.modified().unwrap_or(SystemTime::now())),
        name,
        blocks: metadata.blocks() / 2,
        path: PathBuf::from(path),
    })
}

fn file_name_or_path(path: &Path) -> String {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}

fn format_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut s = String::with_capacity(10);

    s.push(file_type_char(metadata));
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o100 != 0, mode & 0o4000 != 0) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    });

    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o010 != 0, mode & 0o2000 != 0) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    });

    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o001 != 0, mode & 0o1000 != 0) {
        (true, true) => 't',
        (false, true) => 'T',
        (true, false) => 'x',
        (false, false) => '-',
    });

    s
}

fn file_type_char(metadata: &fs::Metadata) -> char {
    let ft = metadata.file_type();

    if ft.is_dir() {
        'd'
    } else if ft.is_symlink() {
        'l'
    } else if ft.is_char_device() {
        'c'
    } else if ft.is_block_device() {
        'b'
    } else if ft.is_fifo() {
        'p'
    } else if ft.is_socket() {
        's'
    } else {
        '-'
    }
}

fn format_size(metadata: &fs::Metadata) -> String {
    if metadata.file_type().is_char_device() || metadata.file_type().is_block_device() {
        let dev = metadata.rdev();
        format!("{}, {}", libc::major(dev), libc::minor(dev))
    } else {
        metadata.len().to_string()
    }
}

fn format_date(modified: SystemTime) -> String {
    let dt: DateTime<Local> = modified.into();
    let dth = dt +Duration::hours(1);
    let now = Local::now();
    let six_months = Duration::days(180);

    if now.signed_duration_since(dt) > six_months {
        dth.format("%b %d  %Y").to_string()
    } else {
        dth.format("%b %d %H:%M").to_string()
    }
}

fn owner_name(metadata: &fs::Metadata) -> Result<String, ShellError> {
    unsafe {
        let pwd = libc::getpwuid(metadata.uid());
        if pwd.is_null() {
            return Err(ShellError::Other("user not found".to_string()));
        }

        let name = CStr::from_ptr((*pwd).pw_name)
            .to_str()
            .map_err(|_| ShellError::Other("invalid UTF-8 in user name".to_string()))?
            .to_string();

        Ok(name)
    }
}

fn group_name(metadata: &fs::Metadata) -> Result<String, ShellError> {
    unsafe {
        let grp = libc::getgrgid(metadata.gid());
        if grp.is_null() {
            return Err(ShellError::Other("group not found".to_string()));
        }

        let name = CStr::from_ptr((*grp).gr_name)
            .to_str()
            .map_err(|_| ShellError::Other("invalid UTF-8 in group name".to_string()))?
            .to_string();

        Ok(name)
    }
}