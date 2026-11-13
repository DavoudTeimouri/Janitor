// Container cleanup — native prune commands for Docker and Podman.
// Stdlib-only: shells out to docker/podman CLIs (same pattern as trash-put/gio).
// Never rm -rf container data; always use native prune commands.

use std::io;
use std::io::Write;
use std::process::Command;

/// Run `docker system prune -a` (removes stopped containers, unused networks,
/// dangling images, and build cache). Returns Ok(()) on success.
pub fn docker_system_prune(yes: bool) -> io::Result<()> {
    if !yes {
        print!("Prune Docker system (stopped containers, unused networks, dangling images, build cache)? [y/N] ");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err()
            || !line.trim_start().eq_ignore_ascii_case("y")
        {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "skipped by user",
            ));
        }
    }
    let status = Command::new("docker").arg("system").arg("prune").arg("-a").status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "docker system prune failed",
        ))
    }
}

/// Run `docker volume prune` (removes unused local volumes).
pub fn docker_volume_prune(yes: bool) -> io::Result<()> {
    if !yes {
        print!("Prune Docker volumes (unused local volumes)? [y/N] ");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err()
            || !line.trim_start().eq_ignore_ascii_case("y")
        {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "skipped by user",
            ));
        }
    }
    let status = Command::new("docker").arg("volume").arg("prune").status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "docker volume prune failed",
        ))
    }
}

/// Run `docker builder prune` (removes build cache).
pub fn docker_builder_prune(yes: bool) -> io::Result<()> {
    if !yes {
        print!("Prune Docker build cache? [y/N] ");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err()
            || !line.trim_start().eq_ignore_ascii_case("y")
        {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "skipped by user",
            ));
        }
    }
    let status = Command::new("docker").arg("builder").arg("prune").status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "docker builder prune failed",
        ))
    }
}

/// Run `podman system prune -a` (Podman equivalent of docker system prune).
pub fn podman_system_prune(yes: bool) -> io::Result<()> {
    if !yes {
        print!("Prune Podman system (stopped containers, unused networks, dangling images, build cache)? [y/N] ");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err()
            || !line.trim_start().eq_ignore_ascii_case("y")
        {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "skipped by user",
            ));
        }
    }
    let status = Command::new("podman").arg("system").arg("prune").arg("-a").status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "podman system prune failed",
        ))
    }
}

/// Check if a CLI binary exists in PATH.
pub fn command_exists(cmd: &str) -> bool {
    std::process::Command::new(cmd).arg("--version").output().is_ok()
}
