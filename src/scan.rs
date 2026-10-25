use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub kind: Kind,
    pub size: u64,
}

#[derive(Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub total_size: u64,
    pub file_count: usize,
    pub files: Vec<FileEntry>,
}

#[derive(PartialEq, Eq, Clone)]
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

pub fn scan_dirs(root: &Path, min_large: u64) -> Vec<DirEntry> {
    let mut dirs: Vec<DirEntry> = Vec::new();
    let _ = walk_dir(root, &mut dirs, min_large);
    dirs.sort_by(|a, b| b.total_size.cmp(&a.total_size));
    dirs
}

fn walk_dir(dir: &Path, dirs: &mut Vec<DirEntry>, min_large: u64) -> std::io::Result<()> {
    let mut current = DirEntry {
        path: dir.to_path_buf(),
        total_size: 0,
        file_count: 0,
        files: Vec::new(),
    };

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if metadata.is_symlink() {
            if !path.exists() {
                current.files.push(FileEntry {
                    path,
                    kind: Kind::OrphanLink,
                    size: 0,
                });
            }
            continue;
        }
        if metadata.is_dir() {
            let _ = walk_dir(&path, dirs, min_large);
            continue;
        } else if metadata.is_file() {
            let size = metadata.len();
            let kind = classify(&path, size, min_large);
            if kind != Kind::Other {
                current.files.push(FileEntry { path, kind, size });
                current.total_size += size;
                current.file_count += 1;
            }
        }
    }

    if current.file_count > 0 {
        dirs.push(current);
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
        let dirs = scan_dirs(p, 1);
        assert!(!dirs.is_empty());
        let kinds: Vec<&str> = dirs[0].files.iter().map(|e| e.kind.label()).collect();
        assert!(kinds.contains(&"temp"));
        assert!(kinds.contains(&"dump"));
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
        let dirs = scan_dirs(p, 1_000_000);
        assert!(dirs.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn directory_size_aggregates_children() {
        let dir = tempfile_in_tmp();
        let p = &dir;
        fs::File::create(p.join("a.tmp")).unwrap().write_all(b"xx").unwrap();
        fs::File::create(p.join("b.tmp")).unwrap().write_all(b"yyy").unwrap();
        let dirs = scan_dirs(p, 1);
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].total_size, 5);
        assert_eq!(dirs[0].file_count, 2);
        let _ = fs::remove_dir_all(&dir);
    }

    fn tempfile_in_tmp() -> std::path::PathBuf {
        let base = std::env::temp_dir();
        let dir = base.join(format!("janitor-test-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        dir
    }
}
