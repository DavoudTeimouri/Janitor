use crate::scan::Entry;
use std::io::{self, Write};

/// Delete every entry without asking.
pub fn remove_all(entries: &[Entry]) {
    let mut removed = 0u64;
    let mut errors = 0u64;
    for e in entries {
        match fs_remove(&e.path) {
            Ok(()) => removed += e.size,
            Err(_) => errors += 1,
        }
    }
    println!("removed {} bytes, {} errors", removed, errors);
}

/// Print each candidate and ask before deleting.
pub fn prompt_remove(entries: &[Entry]) {
    if entries.is_empty() {
        println!("nothing to clean");
        return;
    }
    let mut removed = 0u64;
    for e in entries {
        print!(
            "delete {} ({})? [y/N] ",
            e.path.display(),
            crate::scan::human_size(e.size)
        );
        let _ = io::stdout().flush();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() || !line.trim_start().eq_ignore_ascii_case("y")
        {
            continue;
        }
        match fs_remove(&e.path) {
            Ok(()) => removed += e.size,
            Err(err) => eprintln!("skip {}: {}", e.path.display(), err),
        }
    }
    println!("removed {} bytes", removed);
}

fn fs_remove(path: &std::path::Path) -> io::Result<()> {
    std::fs::remove_file(path)
}
