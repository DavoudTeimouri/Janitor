// Systemd orphan unit detection.
// Finds systemd unit files that exist on disk but are no longer known to systemd,
// or units that systemd knows about but whose files are missing.
// Stdlib-only: shells out to systemctl (same pattern as trash-put/gio).

use std::io;
use std::path::PathBuf;
use std::process::Command;

const SYSTEM_DIRS: &[&str] = &[
    "/etc/systemd/system",
    "/usr/lib/systemd/system",
    "/run/systemd/system",
];

/// List of systemd unit directories to scan.
pub fn systemd_unit_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = SYSTEM_DIRS.iter().map(|d| PathBuf::from(*d)).collect();
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(format!("{home}/.config/systemd/user")));
    }
    dirs
}

/// An orphaned systemd unit.
#[derive(Debug, Clone)]
pub struct OrphanUnit {
    pub name: String,
    pub path: PathBuf,
    pub reason: String,
}

/// Get unit file names that systemd knows about via `systemctl list-unit-files`.
fn systemd_known_units() -> io::Result<Vec<String>> {
    let output = Command::new("systemctl")
        .arg("list-unit-files")
        .arg("--type=service,socket,timer")
        .arg("--no-legend")
        .arg("--no-pager")
        .output()?;
    if !output.status.success() {
        return Ok(Vec::new());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut units: Vec<String> = Vec::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(name) = parts.first() {
            units.push(name.to_string());
        }
    }
    Ok(units)
}

/// Check if `systemctl` is available in PATH.
pub fn systemctl_available() -> bool {
    Command::new("systemctl").arg("--version").output().is_ok()
}

/// Find orphaned systemd units: unit files on disk that systemd does not know about.
pub fn find_orphan_units() -> io::Result<Vec<OrphanUnit>> {
    let mut orphans: Vec<OrphanUnit> = Vec::new();
    let known = systemd_known_units()?;

    for dir in systemd_unit_dirs() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let ext = match path.extension().and_then(|e| e.to_str()) {
                    Some(e) => e,
                    None => continue,
                };
                if ext != "service" && ext != "socket" && ext != "timer" {
                    continue;
                }
                let name = match path.file_stem().and_then(|n| n.to_str()) {
                    Some(n) => n.to_string(),
                    None => continue,
                };
                if !known.iter().any(|u| u == &name) {
                    orphans.push(OrphanUnit {
                        name,
                        path,
                        reason: "unit file exists but not known to systemd".to_string(),
                    });
                }
            }
        }
    }

    Ok(orphans)
}

/// Find failed systemd units via `systemctl list-units --state=failed`.
pub fn find_failed_units() -> io::Result<Vec<String>> {
    let output = Command::new("systemctl")
        .arg("list-units")
        .arg("--type=service,socket,timer")
        .arg("--state=failed")
        .arg("--no-legend")
        .arg("--no-pager")
        .output()?;
    if !output.status.success() {
        return Ok(Vec::new());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut units: Vec<String> = Vec::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(name) = parts.first() {
            if !name.starts_with("UNIT") {
                units.push(name.to_string());
            }
        }
    }
    Ok(units)
}
