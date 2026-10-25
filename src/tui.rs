// TUI — pure stdlib, ANSI escape codes + stty raw mode.
// No external crates. Works on any Linux terminal over SSH.

use crate::clean::clean_files;
use crate::scan::{DirEntry, human_size, scan_dirs};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const HELP: &str =
    "\n\
  ↑/↓  navigate   →/Enter  drill into directory   c  clean selected   q  quit\n\
  r    rescan   /  toggle progress bar   ESC  back to parent\n";

struct State {
    dirs: Vec<DirEntry>,
    idx: usize,
    root: PathBuf,
    show_progress: bool,
    scroll: usize,
    visible: usize,
}

impl State {
    fn new(root: &Path) -> Self {
        Self {
            dirs: Vec::new(),
            idx: 0,
            root: root.to_path_buf(),
            show_progress: true,
            scroll: 0,
            visible: 20,
        }
    }

    fn rescan(&mut self, min_size: u64) {
        self.dirs = scan_dirs(&self.root, min_size);
        self.idx = self.idx.min(self.dirs.len().saturating_sub(1));
        self.scroll = 0;
    }

    fn selected(&self) -> Option<&DirEntry> {
        self.dirs.get(self.idx)
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

fn goto(row: u16, col: u16) {
    print!("\x1b[{};{}H", row + 1, col + 1);
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
    // Save terminal state, switch to raw mode so we read arrow keys byte-by-byte.
    let saved = Command::new("stty").arg("-g").output();
    let raw_ok = Command::new("stty").arg("raw").arg("-echo").arg("min").arg("1").arg("time").arg("0").status();
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

        // Read one key (may be first byte of an escape sequence).
        let first = match bytes.next() {
            Some(Ok(b)) => b,
            _ => break,
        };

        if first == 0x1b {
            // Possible escape sequence: read up to 2 more bytes.
            let second = bytes.next();
            let third = bytes.next();
            // ESC alone or read error → back to parent.
            let go_back = matches!(second, None | Some(Err(_)));
            match (second, third) {
                (Some(Ok(b'[')), Some(Ok(b'A'))) => {
                    // Up
                    if state.idx > 0 {
                        state.idx -= 1;
                        if state.idx < state.scroll {
                            state.scroll = state.idx;
                        }
                    }
                }
                (Some(Ok(b'[')), Some(Ok(b'B'))) => {
                    // Down
                    if state.idx + 1 < state.dirs.len() {
                        state.idx += 1;
                        if state.idx >= state.scroll + state.visible {
                            state.scroll = state.idx - state.visible + 1;
                        }
                    }
                }
                (Some(Ok(b'[')), Some(Ok(b'C'))) => {
                    // Right: drill into selected directory
                    if let Some(d) = state.selected() {
                        state.root = d.path.clone();
                        state.rescan(100_000_000);
                    }
                }
                _ => {}
            }
            if go_back {
                if !state.root.pop() {
                    // already at root, quit
                    break;
                }
                state.rescan(100_000_000);
                continue;
            }
            continue;
        }

        match first {
            b'q' | b'Q' => break,
            b'r' | b'R' => state.rescan(100_000_000),
            b'\r' | b'\n' => {
                // Enter: drill into selected directory
                if let Some(d) = state.selected() {
                    state.root = d.path.clone();
                    state.rescan(100_000_000);
                }
            }
            b'c' | b'C' => {
                // Clean selected directory's files
                if let Some(d) = state.selected() {
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
            b'/' => state.show_progress = !state.show_progress,
            b'b' | b'B' => {
                // Back to parent
                if !state.root.pop() {
                    break;
                }
                state.rescan(100_000_000);
            }
            _ => {}
        }
    }

    show_cursor();
    // Restore terminal
    if let Ok(out) = saved {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let _ = Command::new("stty").arg(s).status();
    }
    Ok(())
}

fn draw(state: &State) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let title = format!("Janitor — {}  ({} dirs)", state.root.display(), state.dirs.len());
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
    writeln!(out, "{}", HELP)?;
    writeln!(out, "Selected: {}", state.selected().map(|d| d.path.display().to_string()).unwrap_or_else(|| "none".to_string()))?;

    out.flush()
}
