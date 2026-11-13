mod clean;
mod docker;
mod orphan;
mod report;
mod rules;
mod scan;
mod systemd;
mod tui;

use std::env;
use std::io::Write;
use std::path::Path;
use std::process;

const USAGE: &str =
    "usage: janitor <scan|clean|orphan|report|drill|tui|docker|podman|systemd> [root] [--min-size N] [--yes]\n\
     scan     - list directories by size under root (default .)\n\
     clean    - delete candidate files (interactive unless --yes)\n\
     orphan   - list broken symlinks under root\n\
     report   - same as scan\n\
     drill <dir> - show files inside a directory sorted by size\n\
     tui      - interactive directory tree explorer (select to scan/clean)\n\
     docker   - prune Docker system, volumes, and build cache\n\
     podman   - prune Podman system\n\
     systemd  - detect and remove orphaned systemd units\
"; // ponytail: hand-rolled arg parsing (stdlib-first, zero deps). Ceiling: ~8 subcommands.
//   Switch to clap when subcommand count or flag complexity justifies a dependency.

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", USAGE);
        process::exit(2);
    }
    let sub = &args[1];
    let root = args.get(2).map(|s| s.as_str()).unwrap_or(".");
    let yes = args.iter().any(|a| a == "--yes");
    let min_size = args
        .iter()
        .position(|a| a == "--min-size")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(100_000_000);

    match sub.as_str() {
        "scan" => report::print(&scan::scan_dirs(root.as_ref(), min_size)),
        "report" => report::print(&scan::scan_dirs(root.as_ref(), min_size)),
        "clean" => {
            let dirs = scan::scan_dirs(root.as_ref(), min_size);
            let files: Vec<_> = dirs.iter().flat_map(|d| d.files.iter().cloned()).collect();
            clean::clean_files(&files, yes);
        }
        "orphan" => report::print_files(&orphan::find(root.as_ref())),
        "drill" => {
            let dirs = scan::scan_dirs(root.as_ref(), min_size);
            let target = Path::new(root);
            if let Some(d) = dirs.iter().find(|d| d.path == target) {
                report::print_files(&d.files);
            } else {
                eprintln!("directory not in scan results: {}", target.display());
                process::exit(1);
            }
        }
        "tui" => {
            let _ = tui::run_tui(Path::new(root));
        }
        "docker" => {
            if !docker::command_exists("docker") {
                eprintln!("docker not found in PATH. Install docker to use this command.");
                process::exit(1);
            }
            if let Err(e) = docker::docker_system_prune(yes) {
                eprintln!("docker system prune: {e}");
            }
            if let Err(e) = docker::docker_volume_prune(yes) {
                eprintln!("docker volume prune: {e}");
            }
            if let Err(e) = docker::docker_builder_prune(yes) {
                eprintln!("docker builder prune: {e}");
            }
        }
        "podman" => {
            if !docker::command_exists("podman") {
                eprintln!("podman not found in PATH. Install podman to use this command.");
                process::exit(1);
            }
            if let Err(e) = docker::podman_system_prune(yes) {
                eprintln!("podman system prune: {e}");
            }
        }
        "systemd" => {
            if !systemd::systemctl_available() {
                eprintln!("systemctl not found in PATH. Install systemd to use this command.");
                process::exit(1);
            }
            match systemd::find_orphan_units() {
                Ok(units) => {
                    if units.is_empty() {
                        println!("no orphaned systemd units found");
                    } else {
                        println!("orphaned systemd units:");
                        for u in &units {
                            println!("  {}  ({})  {}", u.name, u.path.display(), u.reason);
                        }
                        println!();
                        if yes {
                            for u in &units {
                                if let Err(e) = std::fs::remove_file(&u.path) {
                                    eprintln!("remove {}: {e}", u.path.display());
                                } else {
                                    println!("removed {}", u.path.display());
                                }
                            }
                        } else {
                            for u in &units {
                                print!("remove {} ({})? [y/N] ", u.path.display(), u.name);
                                let _ = std::io::stdout().flush();
                                let mut line = String::new();
                                if std::io::stdin().read_line(&mut line).is_ok()
                                    && line.trim_start().eq_ignore_ascii_case("y")
                                {
                                    if let Err(e) = std::fs::remove_file(&u.path) {
                                        eprintln!("remove {}: {e}", u.path.display());
                                    } else {
                                        println!("removed {}", u.path.display());
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("systemd scan failed: {e}"),
            }
            match systemd::find_failed_units() {
                Ok(units) => {
                    if !units.is_empty() {
                        println!();
                        println!("failed systemd units:");
                        for u in &units {
                            println!("  {u}");
                        }
                    }
                }
                Err(e) => eprintln!("systemd failed-units scan failed: {e}"),
            }
        }
        _ => {
            eprintln!("{}", USAGE);
            process::exit(2);
        }
    }
}
