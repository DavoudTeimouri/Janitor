mod clean;
mod orphan;
mod report;
mod scan;

use std::env;
use std::path::Path;
use std::process;

const USAGE: &str = "usage: janitor <scan|clean|orphan|report|drill> [root] [--min-size N] [--yes]\n\
     scan     - list directories by size under root (default .)\n\
     clean    - delete candidate files (interactive unless --yes)\n\
     orphan   - list broken symlinks under root\n\
     report   - same as scan\n\
     drill <dir> - show files inside a directory sorted by size";

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
        _ => {
            eprintln!("{}", USAGE);
            process::exit(2);
        }
    }
}
