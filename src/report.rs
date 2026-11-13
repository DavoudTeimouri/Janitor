use crate::scan::{DirEntry, FileEntry, human_size};

pub fn print(dirs: &[DirEntry], format: &str) {
    if format == "json" {
        print_json(dirs);
        return;
    }
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

fn print_json(dirs: &[DirEntry]) {
    print!("[");
    for (i, d) in dirs.iter().enumerate() {
        let top_kind = d.files.iter().map(|f| f.kind.label()).fold(None, |acc, k| {
            let count = d.files.iter().filter(|f| f.kind.label() == k).count();
            match acc {
                None => Some((k, count)),
                Some((_, c)) if count > c => Some((k, count)),
                Some(x) => Some(x),
            }
        });
        print!("{{\"path\": \"{}\"", json_escape(&d.path.display().to_string()));
        print!(", \"total_size\": {}", d.total_size);
        print!(", \"file_count\": {}", d.file_count);
        if let Some((k, _)) = top_kind {
            print!(", \"top_kind\": \"{}\"", k);
        }
        print!(", \"files\": [");
        for (j, f) in d.files.iter().enumerate() {
            if j > 0 {
                print!(", ");
            }
            print!("{{\"path\": \"{}\", \"kind\": \"{}\", \"size\": {}}}",
                json_escape(&f.path.display().to_string()),
                f.kind.label(),
                f.size);
        }
        print!("]}}");
        if i < dirs.len() - 1 {
            print!(",");
        }
        println!();
    }
    println!("]");
}

pub fn print_files(files: &[FileEntry], format: &str) {
    if format == "json" {
        print_json_files(files);
        return;
    }
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

fn print_json_files(files: &[FileEntry]) {
    println!("[");
    for (i, f) in files.iter().enumerate() {
        print!("  {{\"path\": \"{}\", \"kind\": \"{}\", \"size\": {}}}",
            json_escape(&f.path.display().to_string()),
            f.kind.label(),
            f.size);
        if i < files.len() - 1 {
            print!(",");
        }
        println!();
    }
    println!("]");
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}
