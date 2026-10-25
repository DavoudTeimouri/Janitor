use crate::scan::{DirEntry, FileEntry, human_size};

pub fn print(dirs: &[DirEntry]) {
    if dirs.is_empty() {
        println!("nothing found");
        return;
    }
    println!("{:<60} {:>10} {:>8} {:<14}", "DIRECTORY", "SIZE", "FILES", "TOP TYPE");
    println!("{}", "-".repeat(96));
    let mut total: u64 = 0;
    for d in dirs {
        let top_kind = d.files.iter().map(|f| f.kind.label()).fold(None, |acc, k| {
            let count = d.files.iter().filter(|f| f.kind.label() == k).count();
            match acc {
                None => Some((k, count)),
                Some((_, c)) if count > c => Some((k, count)),
                Some(x) => Some(x),
            }
        });
        println!(
            "{:<60} {:>10} {:>8} {:<14}",
            d.path.display(),
            human_size(d.total_size),
            d.file_count,
            top_kind.map(|(k, _)| k).unwrap_or("-")
        );
        total += d.total_size;
    }
    println!("{}", "-".repeat(96));
    println!(
        "total directories: {}  reclaimable: {}",
        dirs.len(),
        human_size(total)
    );
}

pub fn print_files(files: &[FileEntry]) {
    if files.is_empty() {
        println!("no candidate files");
        return;
    }
    println!("{:<70} {:<14} {:>10}", "PATH", "KIND", "SIZE");
    println!("{}", "-".repeat(98));
    for f in files {
        println!(
            "{:<70} {:<14} {:>10}",
            f.path.display(),
            f.kind.label(),
            human_size(f.size)
        );
    }
    let total: u64 = files.iter().map(|f| f.size).sum();
    println!("{}", "-".repeat(98));
    println!(
        "total files: {}  reclaimable: {}",
        files.len(),
        human_size(total)
    );
}
