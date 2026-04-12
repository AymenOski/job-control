use super::{formatter::{format_long_entries, format_simple_names}, types::LongEntry};

pub fn print_simple_names(names: &[String]) {
    let out = format_simple_names(names);
    print!("{out}");
}

pub fn print_long_entries(entries: &[LongEntry], show_total: bool) {
    let out = format_long_entries(entries, show_total);
    print!("{out}");
}