use crate::scan::{human_size, Entry};

pub fn print(entries: &[Entry]) {
    if entries.is_empty() {
        println!("nothing found");
        return;
    }
    println!("{:<60} {:<14} {:>8}", "PATH", "KIND", "SIZE");
    println!("{}", "-".repeat(86));
    for e in entries {
        println!(
            "{:<60} {:<14} {:>8}",
            e.path.display(),
            e.kind.label(),
            human_size(e.size)
        );
    }
    let total: u64 = entries.iter().map(|e| e.size).sum();
    println!("{}", "-".repeat(86));
    println!(
        "total candidates: {}  reclaimable: {}",
        entries.len(),
        human_size(total)
    );
}
