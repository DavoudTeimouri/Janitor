use crate::scan::{FileEntry, human_size};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(PartialEq, Eq, Clone)]
enum TrashBackend {
    TrashPut,
    GioTrash,
    Rm,
    None,
}

fn detect_backend() -> TrashBackend {
    if Command::new("trash-put").arg("--version").output().is_ok() {
        return TrashBackend::TrashPut;
    }
    if Command::new("gio").arg("--version").output().is_ok() {
        return TrashBackend::GioTrash;
    }
    TrashBackend::None
}

fn remove_one(path: &Path, backend: &TrashBackend, yes: bool) -> io::Result<()> {
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
        TrashBackend::Rm if yes => {
            let status = Command::new("rm").arg("-rf").arg(path).status()?;
            if status.success() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "rm failed"))
            }
        }
        TrashBackend::Rm => Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "permanent delete requires --yes",
        )),
        TrashBackend::None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no trash backend available (install trash-cli or gvfs)",
        )),
    }
}

pub fn clean_files(files: &[FileEntry], yes: bool) {
    let backend = detect_backend();
    if files.is_empty() {
        println!("nothing to clean");
        return;
    }
    let mut removed = 0u64;
    let mut errors = 0u64;
    for e in files {
        let proceed = if yes {
            true
        } else {
            print!("delete {} ({})? [y/N] ", e.path.display(), human_size(e.size));
            let _ = io::stdout().flush();
            let mut line = String::new();
            io::stdin().read_line(&mut line).is_ok()
                && line.trim_start().eq_ignore_ascii_case("y")
        };
        if !proceed {
            continue;
        }
        match remove_one(&e.path, &backend, yes) {
            Ok(()) => removed += e.size,
            Err(err) => {
                errors += 1;
                eprintln!("skip {}: {}", e.path.display(), err);
            }
        }
    }
    println!("removed {} bytes, {} errors", removed, errors);
}
