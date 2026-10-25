use crate::scan::{Entry, Kind};
use std::path::Path;

/// Find broken symlinks under `root`.
pub fn find(root: &Path) -> Vec<Entry> {
    let mut out: Vec<Entry> = Vec::new();
    let _ = walk(root, &mut out);
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

fn walk(dir: &Path, out: &mut Vec<Entry>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if metadata.is_symlink() && !path.exists() {
            out.push(Entry {
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
