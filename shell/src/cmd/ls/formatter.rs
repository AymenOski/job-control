use std::fs::Metadata;
use super::types::LongEntry;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
pub fn append_indicator(name: &mut String, metadata: &Metadata) {
    if metadata.file_type().is_dir() {
        name.push('/');
        
    } else if metadata.file_type().is_symlink() {
        name.push('@');
    } else if metadata.file_type().is_fifo() {
        name.push('|');
    } else if metadata.file_type().is_socket() {
        name.push('=');
    } else if (metadata.permissions().mode() & 0o111) != 0 {
        name.push('*');
    }
}

pub fn format_long_entries(entries: &[LongEntry], show_total: bool) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut w_links = 0;
    let mut w_user = 0;
    let mut w_group = 0;
    let mut w_size = 0;
    let mut w_date = 0;
    let mut total_blocks = 0;

    for e in entries {
        w_links = w_links.max(e.links.len());
        w_user = w_user.max(e.user.len());
        w_group = w_group.max(e.group.len());
        w_size = w_size.max(e.size.len());
        w_date = w_date.max(e.date.len());
        total_blocks += e.blocks;
    }

    let mut out = String::new();

    if show_total {
        out.push_str(&format!("total {}\n", total_blocks));
    }

    for e in entries {
        out.push_str(&format!(
            "{} {:>lw$} {:<uw$} {:<gw$} {:>sw$} {:>dw$} {}\n",
            e.perms,
            e.links,
            e.user,
            e.group,
            e.size,
            e.date,
            e.name,
            lw = w_links,
            uw = w_user,
            gw = w_group,
            sw = w_size,
            dw = w_date
        ));
    }

    out
}

pub fn format_simple_names(names: &[String]) -> String {
    if names.is_empty() {
        String::new()
    } else {
        format!("{}\n", names.join("  "))
    }
}