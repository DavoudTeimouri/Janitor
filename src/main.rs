mod clean;
mod orphan;
mod report;
mod scan;

use std::env;
use std::process;

const USAGE: &str = "usage: janitor <scan|clean|orphan|report> [root] [--yes]\n\
     scan   - list unnecessary files under root (default .)\n\
     clean  - delete listed files (interactive unless --yes)\n\
     orphan - list broken symlinks under root\n\
     report - same as scan, human readable";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", USAGE);
        process::exit(2);
    }
    let sub = &args[1];
    let root = args.get(2).map(|s| s.as_str()).unwrap_or(".");
    let yes = args.iter().any(|a| a == "--yes");
    match sub.as_str() {
        "scan" => report::print(&scan::scan(root.as_ref(), 100_000_000)),
        "clean" => {
            let entries = scan::scan(root.as_ref(), 100_000_000);
            if yes {
                clean::remove_all(&entries);
            } else {
                clean::prompt_remove(&entries);
            }
        }
        "orphan" => report::print(&orphan::find(root.as_ref())),
        "report" => report::print(&scan::scan(root.as_ref(), 100_000_000)),
        _ => {
            eprintln!("{}", USAGE);
            process::exit(2);
        }
    }
}
