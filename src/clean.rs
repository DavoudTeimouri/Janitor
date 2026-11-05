use crate::scan::{FileEntry, human_size, is_protected};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum TrashBackend {
    TrashPut,
    GioTrash,
    Rm,
    None,
}

fn detect_backend(allow_rm: bool) -> TrashBackend {
    if Command::new("trash-put").arg("--version").output().is_ok() {
        return TrashBackend::TrashPut;
    }
    if Command::new("gio").arg("--version").output().is_ok() {
        return TrashBackend::GioTrash;
    }
    if allow_rm {
        return TrashBackend::Rm;
    }
    TrashBackend::None
}

fn remove_one(path: &Path, backend: &TrashBackend) -> io::Result<()> {
    match backend {
        TrashBackend::TrashPut => {
            let status = Command::new("trash-put").arg(path).status()?;
            if status.success() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "trash-put failed"))
            }
        }
        TrashBackend::GioTrash => {
            let status = Command::new("gio").arg("trash").arg(path).status()?;
            if status.success() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "gio trash failed"))
            }
        }
        TrashBackend::Rm => {
            if is_protected(path) {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "protected path — refused to delete",
                ));
            }
            let status = Command::new("rm").arg("-rf").arg(path).status()?;
            if status.success() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "rm failed"))
            }
        }
        TrashBackend::None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no trash backend available (install trash-cli or gvfs)",
        )),
    }
}

/// Prompt user (unless --yes), then remove via the configured backend.
/// Returns Interrupted error if user declined.
pub(crate) fn clean_one(path: &Path, size: u64, backend: &TrashBackend, yes: bool) -> io::Result<()> {
    if matches!(backend, TrashBackend::Rm) && is_protected(path) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "protected path — refused to delete",
        ));
    }
    if !yes {
        print!("delete {} ({})? [y/N] ", path.display(), human_size(size));
        let _ = io::stdout().flush();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() || !line.trim_start().eq_ignore_ascii_case("y") {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "skipped by user"));
        }
    }
    remove_one(path, backend)
}

pub fn clean_files(files: &[FileEntry], yes: bool) {
    let backend = detect_backend(yes);
    if files.is_empty() {
        println!("nothing to clean");
        return;
    }
    let mut removed = 0u64;
    let mut errors = 0u64;
    let mut skipped_protected = 0u64;
    for e in files {
        if matches!(backend, TrashBackend::Rm) && is_protected(&e.path) {
            skipped_protected += 1;
            eprintln!("skip {}: protected path", e.path.display());
            continue;
        }
        match clean_one(&e.path, e.size, &backend, yes) {
            Ok(()) => removed += e.size,
            Err(err) => {
                if err.kind() != io::ErrorKind::Interrupted {
                    errors += 1;
                    eprintln!("skip {}: {}", e.path.display(), err);
                }
            }
        }
    }
    println!("removed {} bytes, {} errors", removed, errors);
    if skipped_protected > 0 {
        println!("skipped {} protected paths", skipped_protected);
    }
}
