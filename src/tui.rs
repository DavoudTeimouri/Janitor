// TUI — pure stdlib, ANSI escape codes + stty raw mode.
// No external crates. Works on any Linux terminal over SSH.

use crate::clean::clean_files;
use crate::scan::{DirEntry, FileEntry, human_size, scan_dirs};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const HELP_DIRS: &str =
    "\n\
  ↑/↓  navigate   →/Enter  drill into files   c  clean dir's files   q  quit\n\
  r    rescan   /  toggle progress   ESC  back to parent";

const HELP_FILES: &str =
    "\n\
  ↑/↓  navigate   c  clean selected   d  delete selected   g  back to dirs\n\
  q    quit";

#[derive(PartialEq, Eq, Clone, Copy)]
enum ViewMode {
    Dirs,
    Files,
}

struct State {
    mode: ViewMode,
    dirs: Vec<DirEntry>,
    files: Vec<FileEntry>,
    idx: usize,
    root: PathBuf,
    show_progress: bool,
    scroll: usize,
    visible: usize,
    scanning: bool,
}

impl State {
    fn new(root: &Path) -> Self {
        Self {
            mode: ViewMode::Dirs,
            dirs: Vec::new(),
            files: Vec::new(),
            idx: 0,
            root: root.to_path_buf(),
            show_progress: true,
            scroll: 0,
            visible: 20,
            scanning: false,
        }
    }

    fn rescan(&mut self, min_size: u64) {
        self.scanning = true;
        self.dirs = scan_dirs(&self.root, min_size);
        self.scanning = false;
        self.idx = self.idx.min(self.dirs.len().saturating_sub(1));
        self.scroll = 0;
        self.files.clear();
    }

    fn selected_dir(&self) -> Option<&DirEntry> {
        self.dirs.get(self.idx)
    }

    fn selected_file(&self) -> Option<&FileEntry> {
        self.files.get(self.idx)
    }

    fn open_selected_dir(&mut self) {
        if let Some(d) = self.selected_dir() {
            self.files = d.files.clone();
            self.idx = 0;
            self.scroll = 0;
            self.mode = ViewMode::Files;
        }
    }

    fn back_to_dirs(&mut self) {
        self.mode = ViewMode::Dirs;
        self.idx = 0;
        self.scroll = 0;
        self.files.clear();
    }
}

fn clear() {
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}

fn hide_cursor() {
    print!("\x1b[?25l");
    let _ = io::stdout().flush();
}

fn show_cursor() {
    print!("\x1b[?25h");
    let _ = io::stdout().flush();
}

fn progress_bar(width: usize, done: u64, total: u64) -> String {
    if total == 0 {
        return format!("[{}]", " ".repeat(width));
    }
    let filled = (done as usize * width) / total as usize;
    let rest = width.saturating_sub(filled);
    format!("[{}{}] {:>3}%", "#".repeat(filled), "-".repeat(rest), done * 100 / total)
}

/// Enter TUI mode. Returns Ok(()) on normal quit.
pub fn run_tui(root: &Path) -> io::Result<()> {
    let saved = Command::new("stty").arg("-g").output();
    let raw_ok = Command::new("stty")
        .arg("raw")
        .arg("-echo")
        .arg("min")
        .arg("1")
        .arg("time")
        .arg("0")
        .status();
    if saved.is_err() || raw_ok.is_err() {
        eprintln!("TUI requires a terminal with stty; falling back to CLI.");
        eprintln!("Use: janitor scan <root> | janitor drill <dir> | janitor clean <root> --yes");
        return Ok(());
    }

    hide_cursor();
    let mut state = State::new(root);
    state.rescan(100_000_000);

    let stdin = io::stdin();
    let mut bytes = stdin.bytes();

    loop {
        clear();
        draw(&state)?;

        let first = match bytes.next() {
            Some(Ok(b)) => b,
            _ => break,
        };

        if first == 0x1b {
            let second = bytes.next();
            let third = bytes.next();
            let go_back = matches!(second, None | Some(Err(_)));
            match (second, third) {
                (Some(Ok(b'[')), Some(Ok(b'A'))) => {
                    if state.idx > 0 {
                        state.idx -= 1;
                        if state.idx < state.scroll {
                            state.scroll = state.idx;
                        }
                    }
                }
                (Some(Ok(b'[')), Some(Ok(b'B'))) => {
                    let len = match state.mode {
                        ViewMode::Dirs => state.dirs.len(),
                        ViewMode::Files => state.files.len(),
                    };
                    if state.idx + 1 < len {
                        state.idx += 1;
                        if state.idx >= state.scroll + state.visible {
                            state.scroll = state.idx - state.visible + 1;
                        }
                    }
                }
                (Some(Ok(b'[')), Some(Ok(b'C'))) => {
                    if state.mode == ViewMode::Dirs {
                        state.open_selected_dir();
                    }
                }
                _ => {}
            }
            if go_back {
                match state.mode {
                    ViewMode::Files => state.back_to_dirs(),
                    ViewMode::Dirs => {
                        if !state.root.pop() {
                            break;
                        }
                        state.rescan(100_000_000);
                    }
                }
                continue;
            }
            continue;
        }

        match first {
            b'q' | b'Q' => break,
            b'r' | b'R' => state.rescan(100_000_000),
            b'\r' | b'\n' => {
                if state.mode == ViewMode::Dirs {
                    state.open_selected_dir();
                }
            }
            b'c' | b'C' => match state.mode {
                ViewMode::Dirs => {
                    if let Some(d) = state.selected_dir() {
                        show_cursor();
                        clear();
                        clean_files(&d.files, false);
                        println!("\nPress any key to return...");
                        let _ = bytes.next();
                        hide_cursor();
                        clear();
                        state.rescan(100_000_000);
                    }
                }
                ViewMode::Files => {
                    if let Some(f) = state.selected_file() {
                        show_cursor();
                        clear();
                        clean_files(&[f.clone()], false);
                        println!("\nPress any key to return...");
                        let _ = bytes.next();
                        hide_cursor();
                        clear();
                        state.rescan(100_000_000);
                    }
                }
            },
            b'd' | b'D' => {
                if state.mode == ViewMode::Files {
                    if let Some(f) = state.selected_file() {
                        show_cursor();
                        clear();
                        print!("PERMANENTLY delete {}? [y/N] ", f.path.display());
                        let _ = io::stdout().flush();
                        let mut line = String::new();
                        if io::stdin().read_line(&mut line).is_ok()
                            && line.trim_start().eq_ignore_ascii_case("y")
                        {
                            match std::fs::remove_file(&f.path) {
                                Ok(()) => println!("deleted {}", f.path.display()),
                                Err(err) => eprintln!("skip {}: {}", f.path.display(), err),
                            }
                        }
                        println!("\nPress any key to return...");
                        let _ = bytes.next();
                        hide_cursor();
                        clear();
                        state.rescan(100_000_000);
                    }
                }
            }
            b'g' | b'G' => {
                if state.mode == ViewMode::Files {
                    state.back_to_dirs();
                }
            }
            b'/' => state.show_progress = !state.show_progress,
            b'b' | b'B' => {
                if state.mode == ViewMode::Dirs {
                    if !state.root.pop() {
                        break;
                    }
                    state.rescan(100_000_000);
                }
            }
            _ => {}
        }
    }

    show_cursor();
    if let Ok(out) = saved {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let _ = Command::new("stty").arg(s).status();
    }
    Ok(())
}

fn draw(state: &State) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mode_label = match state.mode {
        ViewMode::Dirs => "DIRECTORIES",
        ViewMode::Files => "FILES",
    };
    let title = format!(
        "Janitor — {}  [{}]  ({} dirs)",
        state.root.display(),
        mode_label,
        state.dirs.len()
    );
    writeln!(out, "\x1b[1m{}\x1b[0m", title)?;
    writeln!(out, "{}", "-".repeat(title.len()))?;

    if state.dirs.is_empty() {
        writeln!(out, "No candidate directories found.")?;
    } else {
        let end = (state.scroll + state.visible).min(state.dirs.len());
        for i in state.scroll..end {
            let d = &state.dirs[i];
            let marker = if i == state.idx { "▶" } else { "  " };
            let line = if i == state.idx {
                format!("\x1b[7m{} {}  {:>8}  {}\x1b[0m", marker, d.path.display(), human_size(d.total_size), d.file_count)
            } else {
                format!("{} {}  {:>8}  {}", marker, d.path.display(), human_size(d.total_size), d.file_count)
            };
            writeln!(out, "{}", line)?;
        }
        if end < state.dirs.len() {
            writeln!(out, "\x1b[2m... {} more dirs\x1b[0m", state.dirs.len() - end)?;
        }
    }

    writeln!(out)?;
    if state.show_progress {
        writeln!(out, "Scan complete. {}", progress_bar(40, state.dirs.len() as u64, state.dirs.len() as u64))?;
    }
    writeln!(out, "{}", match state.mode {
        ViewMode::Dirs => HELP_DIRS,
        ViewMode::Files => HELP_FILES,
    })?;
    writeln!(
        out,
        "Selected: {}",
        match state.mode {
            ViewMode::Dirs => state
                .selected_dir()
                .map(|d| d.path.display().to_string())
                .unwrap_or_else(|| "none".to_string()),
            ViewMode::Files => state
                .selected_file()
                .map(|f| f.path.display().to_string())
                .unwrap_or_else(|| "none".to_string()),
        }
    )?;

    out.flush()
}
