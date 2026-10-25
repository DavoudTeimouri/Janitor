use crate::scan::{FileEntry, Kind};
use std::path::Path;

/// Find broken symlinks under `root`.
pub fn find(root: &Path) -> Vec<FileEntry> {
    let mut out: Vec<FileEntry> = Vec::new();
    let _ = walk(root, &mut out);
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

fn walk(dir: &Path, out: &mut Vec<FileEntry>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if metadata.is_symlink() && !path.exists() {
            out.push(FileEntry {
                path,
                kind: Kind::OrphanLink,
                size: 0,
            });
            continue;
        }
        if metadata.is_dir() {
            let _ = walk(&path, out);
        }
    }
    Ok(())
}
