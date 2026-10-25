use std::fs;
use std::path::{Path, PathBuf};

pub struct Entry {
    pub path: PathBuf,
    pub kind: Kind,
    pub size: u64,
}

#[derive(PartialEq, Eq)]
pub enum Kind {
    Temp,
    Dump,
    Cache,
    Log,
    Large,
    OrphanLink,
    Other,
}

impl Kind {
    pub fn label(&self) -> &'static str {
        match self {
            Kind::Temp => "temp",
            Kind::Dump => "dump",
            Kind::Cache => "cache",
            Kind::Log => "log",
            Kind::Large => "large",
            Kind::OrphanLink => "broken-link",
            Kind::Other => "other",
        }
    }
}

/// Walk `root` and collect unnecessary entries. Sorted by size descending.
pub fn scan(root: &Path, min_large: u64) -> Vec<Entry> {
    let mut out: Vec<Entry> = Vec::new();
    let _ = walk(root, &mut out, min_large);
    out.sort_by(|a, b| b.size.cmp(&a.size));
    out
}

fn walk(dir: &Path, out: &mut Vec<Entry>, min_large: u64) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if metadata.is_symlink() {
            if !path.exists() {
                out.push(Entry {
                    path,
                    kind: Kind::OrphanLink,
                    size: 0,
                });
            }
            continue;
        }
        if metadata.is_dir() {
            let _ = walk(&path, out, min_large);
        } else if metadata.is_file() {
            let size = metadata.len();
            let kind = classify(&path, size, min_large);
            if kind != Kind::Other {
                out.push(Entry { path, kind, size });
            }
        }
    }
    Ok(())
}

fn classify(path: &Path, size: u64, min_large: u64) -> Kind {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let lower = name.to_lowercase();
    let spill = path.to_string_lossy();
    if lower.ends_with(".tmp") || lower.ends_with(".temp") {
        return Kind::Temp;
    }
    if lower.ends_with(".dmp")
        || lower.ends_with(".dump")
        || lower.ends_with(".crash")
        || spill.contains("/var/crash/")
    {
        return Kind::Dump;
    }
    if lower.contains("cache") || spill.contains("/.cache/") {
        return Kind::Cache;
    }
    if lower.ends_with(".log") || lower.ends_with(".log.old") || spill.contains("/var/log/") {
        return Kind::Log;
    }
    if size >= min_large {
        return Kind::Large;
    }
    Kind::Other
}

pub fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut idx = 0;
    while size >= 1024.0 && idx < UNITS.len() - 1 {
        size /= 1024.0;
        idx += 1;
    }
    if idx == 0 {
        format!("{} {}", bytes, UNITS[idx])
    } else {
        format!("{:.2} {}", size, UNITS[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn finds_temp_dump_and_large() {
        let dir = tempfile_in_tmp();
        let p = &dir;
        fs::File::create(p.join("a.tmp"))
            .unwrap()
            .write_all(b"x")
            .unwrap();
        fs::File::create(p.join("core.dump"))
            .unwrap()
            .write_all(b"y")
            .unwrap();
        fs::File::create(p.join("keep.txt"))
            .unwrap()
            .write_all(b"z")
            .unwrap();
        let entries = scan(p, 1);
        let kinds: Vec<&str> = entries.iter().map(|e| e.kind.label()).collect();
        assert!(kinds.contains(&"temp"));
        assert!(kinds.contains(&"dump"));
        assert!(!kinds.contains(&"other"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn skips_other_files_by_default() {
        let dir = tempfile_in_tmp();
        let p = &dir;
        fs::File::create(p.join("keep.txt"))
            .unwrap()
            .write_all(b"z")
            .unwrap();
        let entries = scan(p, 1_000_000);
        assert!(entries.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    fn tempfile_in_tmp() -> std::path::PathBuf {
        let base = std::env::temp_dir();
        let dir = base.join(format!("janitor-test-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        dir
    }
}
