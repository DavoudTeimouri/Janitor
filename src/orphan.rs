use crate::scan::{FileEntry, Kind, is_protected};
use std::path::Path;

/// Find broken symlinks under `root`. Skips protected paths.
pub fn find(root: &Path) -> Vec<FileEntry> {
    let mut out: Vec<FileEntry> = Vec::new();
    let _ = walk(root, &mut out);
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

fn walk(dir: &Path, out: &mut Vec<FileEntry>) -> std::io::Result<()> {
    if is_protected(dir) {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if metadata.is_symlink() && !path.exists() && !is_protected(&path) {
            out.push(FileEntry {
                path,
                kind: Kind::OrphanLink,
                size: 0,
            });
            continue;
        }
        if metadata.is_dir() {
            if !is_protected(&path) {
                let _ = walk(&path, out);
            }
        }
    }
    Ok(())
}
